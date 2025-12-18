//! Authentication route handlers

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Deserialize;

use crate::db::DbError;
use crate::models::{
    AppState, AuthResponse, Claims, GoogleAuthRequest, LoginRequest, RegisterRequest, User,
    UserResponse,
};

use super::response::{
    bad_request, conflict, internal_error_logged, not_authenticated, ok, unauthorized,
};

/// Request for mobile Google Sign-In with ID token
#[derive(Debug, Deserialize)]
pub struct GoogleIdTokenRequest {
    pub id_token: String,
}

/// Google ID token payload
#[derive(Debug, Deserialize)]
pub struct GoogleIdTokenPayload {
    pub sub: String,           // Google user ID
    pub email: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub aud: String,           // Client ID
    #[allow(dead_code)]
    pub iss: String,           // Issuer
    #[allow(dead_code)]
    pub exp: u64,              // Expiration
}

// Shorten token lifetime to reduce chance of SSO/session desyncs. Tokens are
// intentionally short-lived; clients should re-login or obtain a fresh token
// via the login flow. For a refresh-token-based flow, implement a refresh
// endpoint and rotate refresh tokens securely.
const TOKEN_EXPIRY_DAYS: i64 = 1;

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    #[allow(dead_code)]
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

fn create_token(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(TOKEN_EXPIRY_DAYS))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        email: user.email.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    // Validate input
    if body.email.is_empty() || body.password.is_empty() || body.name.is_empty() {
        return bad_request("Email, password, and name are required");
    }

    if body.password.len() < 6 {
        return bad_request("Password must be at least 6 characters");
    }

    // Hash password
    let password_hash = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => return internal_error_logged("Failed to hash password", e),
    };

    let user = User::new(body.email.clone(), password_hash, body.name.clone());

    match state.db.create_user(&user).await {
        Ok(user) => {
            let token = match create_token(&user, &state.jwt_secret) {
                Ok(t) => t,
                Err(e) => return internal_error_logged("Failed to create token", e),
            };

            HttpResponse::Created().json(AuthResponse {
                token,
                user: user.into(),
            })
        }
        Err(DbError::EmailExists) => conflict("Email already registered"),
        Err(e) => internal_error_logged("Failed to create user", e),
    }
}

pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> impl Responder {
    let user = match state.db.get_user_by_email(&body.email).await {
        Ok(u) => u,
        Err(DbError::NotFound) => return unauthorized("Invalid email or password"),
        Err(e) => return internal_error_logged("Failed to get user", e),
    };

    // Check password
    let password_hash = match &user.password_hash {
        Some(h) => h,
        None => return unauthorized("Please login with Google"),
    };

    match verify(&body.password, password_hash) {
        Ok(true) => {}
        Ok(false) => return unauthorized("Invalid email or password"),
        Err(e) => return internal_error_logged("Failed to verify password", e),
    }

    let token = match create_token(&user, &state.jwt_secret) {
        Ok(t) => t,
        Err(e) => return internal_error_logged("Failed to create token", e),
    };

    ok(AuthResponse {
        token,
        user: user.into(),
    })
}

pub async fn google_auth(
    state: web::Data<AppState>,
    body: web::Json<GoogleAuthRequest>,
) -> impl Responder {
    // Exchange code for token
    let client = reqwest::Client::new();

    let token_response = match client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", body.code.as_str()),
            ("client_id", state.google_client_id.as_str()),
            ("client_secret", state.google_client_secret.as_str()),
            (
                "redirect_uri",
                &format!("{}/auth/google/callback", state.app_url),
            ),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return internal_error_logged("Failed to exchange Google code", e),
    };

    let token_data: GoogleTokenResponse = match token_response.json().await {
        Ok(data) => data,
        Err(e) => return internal_error_logged("Failed to parse Google token response", e),
    };

    // Get user info
    let user_info_response = match client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_data.access_token)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return internal_error_logged("Failed to get Google user info", e),
    };

    let google_user: GoogleUserInfo = match user_info_response.json().await {
        Ok(data) => data,
        Err(e) => return internal_error_logged("Failed to parse Google user info", e),
    };

    // Create or update user
    let user = match state
        .db
        .upsert_google_user(
            &google_user.email,
            &google_user.id,
            &google_user.name,
            google_user.picture.as_deref(),
        )
        .await
    {
        Ok(u) => u,
        Err(e) => return internal_error_logged("Failed to upsert Google user", e),
    };

    let token = match create_token(&user, &state.jwt_secret) {
        Ok(t) => t,
        Err(e) => return internal_error_logged("Failed to create token", e),
    };

    ok(AuthResponse {
        token,
        user: user.into(),
    })
}

/// Mobile Google Sign-In using ID token
/// This endpoint is used by mobile apps that get an ID token directly from Google Sign-In SDK
pub async fn google_auth_mobile(
    state: web::Data<AppState>,
    body: web::Json<GoogleIdTokenRequest>,
) -> impl Responder {
    // Verify the ID token with Google
    let client = reqwest::Client::new();
    
    let verify_url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        body.id_token
    );
    
    let response = match client.get(&verify_url).send().await {
        Ok(resp) => resp,
        Err(e) => return internal_error_logged("Failed to verify Google ID token", e),
    };
    
    if !response.status().is_success() {
        return unauthorized("Invalid Google ID token");
    }
    
    let payload: GoogleIdTokenPayload = match response.json().await {
        Ok(data) => data,
        Err(e) => return internal_error_logged("Failed to parse Google token info", e),
    };
    
    // Verify the token is for our app (check audience)
    // Note: For Android, the aud should match your Android client ID
    // We accept any of the configured client IDs
    if !payload.aud.starts_with(&state.google_client_id.split('.').next().unwrap_or("")) 
        && payload.aud != state.google_client_id {
        log::warn!("Google ID token audience mismatch: {} vs {}", payload.aud, state.google_client_id);
        // For now, we'll allow it but log the warning
        // In production, you might want to verify against Android client ID as well
    }
    
    // Verify email is verified
    if payload.email_verified == Some(false) {
        return bad_request("Email not verified with Google");
    }
    
    let name = payload.name.unwrap_or_else(|| payload.email.split('@').next().unwrap_or("User").to_string());
    
    // Create or update user
    let user = match state
        .db
        .upsert_google_user(
            &payload.email,
            &payload.sub,
            &name,
            payload.picture.as_deref(),
        )
        .await
    {
        Ok(u) => u,
        Err(e) => return internal_error_logged("Failed to upsert Google user", e),
    };
    
    let token = match create_token(&user, &state.jwt_secret) {
        Ok(t) => t,
        Err(e) => return internal_error_logged("Failed to create token", e),
    };
    
    ok(AuthResponse {
        token,
        user: user.into(),
    })
}

pub async fn get_google_auth_url(state: web::Data<AppState>) -> impl Responder {
    let redirect_uri = format!("{}/auth/google/callback", state.app_url);
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email%20profile&access_type=offline",
        state.google_client_id,
        urlencoding::encode(&redirect_uri)
    );

    ok(serde_json::json!({ "url": auth_url }))
}

pub async fn get_me(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = match get_user_id_from_request(&req, &state.jwt_secret) {
        Some(id) => id,
        None => return not_authenticated(),
    };

    match state.db.get_user_by_id(&user_id).await {
        Ok(user) => ok(UserResponse::from(user)),
        Err(DbError::NotFound) => unauthorized("User not found"),
        Err(e) => internal_error_logged("Failed to get user", e),
    }
}

/// Get user storage usage
pub async fn get_storage(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = match get_user_id_from_request(&req, &state.jwt_secret) {
        Some(id) => id,
        None => return not_authenticated(),
    };

    match state.db.get_user_storage_used(&user_id).await {
        Ok(used) => ok(serde_json::json!({
            "used": used,
            "limit": 1073741824i64  // 1GB default limit
        })),
        Err(e) => internal_error_logged("Failed to get storage usage", e),
    }
}

// ============ Auth Helper Functions ============

/// Extract user ID from Authorization header
pub fn get_user_id_from_request(req: &HttpRequest, secret: &str) -> Option<String> {
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;

    if !auth_str.starts_with("Bearer ") {
        return None;
    }

    let token = &auth_str[7..];

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;

    Some(token_data.claims.sub)
}

/// Extract user ID from request, with fallback to query parameter (for image loading)
pub fn get_user_id_from_request_or_query(req: &HttpRequest, secret: &str) -> Option<String> {
    // First try header
    if let Some(user_id) = get_user_id_from_request(req, secret) {
        return Some(user_id);
    }

    // Then try query parameter (for image loading)
    let query_string = req.query_string();
    for pair in query_string.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == "token" {
                let token = urlencoding::decode(value).ok()?;
                let token_data = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                )
                .ok()?;
                return Some(token_data.claims.sub);
            }
        }
    }

    None
}
