use super::{Database, DbError, DbResult};
use crate::models::User;
use chrono::Utc;

impl Database {
    pub async fn get_user_by_id(&self, id: &str) -> DbResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT id, email, password_hash, name, google_id, avatar_url, created_at, updated_at 
               FROM users WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> DbResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT id, email, password_hash, name, google_id, avatar_url, created_at, updated_at 
               FROM users WHERE email = $1"#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_google_id(&self, google_id: &str) -> DbResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT id, email, password_hash, name, google_id, avatar_url, created_at, updated_at 
               FROM users WHERE google_id = $1"#
        )
        .bind(google_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> DbResult<User> {
        // Check if email already exists
        if let Ok(_) = self.get_user_by_email(&user.email).await {
            return Err(DbError::EmailExists);
        }

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, name, google_id, avatar_url, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.google_id)
        .bind(&user.avatar_url)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(user.clone())
    }

    #[allow(dead_code)]
    pub async fn update_user(
        &self,
        id: &str,
        name: Option<String>,
        avatar_url: Option<String>,
    ) -> DbResult<User> {
        let existing = self.get_user_by_id(id).await?;

        let new_name = name.unwrap_or(existing.name);
        let new_avatar = avatar_url.or(existing.avatar_url);
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE users SET name = $1, avatar_url = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(&new_name)
        .bind(&new_avatar)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_user_by_id(id).await
    }

    pub async fn upsert_google_user(
        &self,
        email: &str,
        google_id: &str,
        name: &str,
        avatar_url: Option<&str>,
    ) -> DbResult<User> {
        // Try to find by google_id first
        if let Ok(user) = self.get_user_by_google_id(google_id).await {
            return Ok(user);
        }

        // Try to find by email and link google account
        if let Ok(mut user) = self.get_user_by_email(email).await {
            sqlx::query(
                "UPDATE users SET google_id = $1, avatar_url = COALESCE($2, avatar_url), updated_at = $3 WHERE id = $4"
            )
            .bind(google_id)
            .bind(avatar_url)
            .bind(Utc::now())
            .bind(&user.id)
            .execute(&self.pool)
            .await?;

            user.google_id = Some(google_id.to_string());
            if let Some(url) = avatar_url {
                user.avatar_url = Some(url.to_string());
            }
            return Ok(user);
        }

        // Create new user
        let user = User::new_google(email.to_string(), google_id.to_string(), name.to_string(), avatar_url.map(|s| s.to_string()));
        self.create_user(&user).await
    }
}
