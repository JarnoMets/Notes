//! Sync routes for mobile app
//!
//! Provides efficient endpoints for mobile apps to sync all data in fewer requests.

use crate::models::AppState;
use crate::require_auth;
use crate::routes::response;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Full sync response containing all user data
#[derive(Debug, Serialize)]
pub struct FullSyncResponse {
    pub user: UserInfo,
    pub notes: NotesSync,
    pub boards: BoardsSync,
    pub synced_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NotesSync {
    pub folders: Vec<FolderSync>,
    pub notes: Vec<NoteSync>,
}

#[derive(Debug, Serialize)]
pub struct FolderSync {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub position: i32,
    pub is_important: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NoteSync {
    pub id: String,
    pub title: String,
    pub content: String,
    pub folder_id: Option<String>,
    pub position: i32,
    pub is_important: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BoardsSync {
    pub folders: Vec<BoardFolderSync>,
    pub boards: Vec<BoardSync>,
}

#[derive(Debug, Serialize)]
pub struct BoardFolderSync {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BoardSync {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub folder_id: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub lists: Vec<ListSync>,
    pub labels: Vec<LabelSync>,
}

#[derive(Debug, Serialize)]
pub struct ListSync {
    pub id: String,
    pub name: String,
    pub position: i32,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cards: Vec<CardSync>,
}

#[derive(Debug, Serialize)]
pub struct LabelSync {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct CardSync {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub labels: Vec<String>,
}

/// Full sync - returns all user data
/// 
/// This endpoint is designed for initial sync or when the mobile app needs
/// to refresh all data. It returns everything in a single request.
pub async fn full_sync(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let user_id = require_auth!(req, state);

    // Get user info
    let user = match state.db.get_user_by_id(&user_id).await {
        Ok(u) => UserInfo {
            id: u.id,
            email: u.email,
            name: u.name,
            avatar_url: u.avatar_url,
        },
        Err(e) => return response::internal_error_logged("Failed to get user", e),
    };

    // Get all note folders
    let note_folders = match state.db.get_folders(&user_id).await {
        Ok(folders) => folders
            .into_iter()
            .map(|f| FolderSync {
                id: f.id,
                name: f.name,
                parent_id: f.parent_id,
                position: f.position,
                is_important: f.is_important,
                created_at: f.created_at,
                updated_at: f.updated_at,
            })
            .collect(),
        Err(e) => return response::internal_error_logged("Failed to get folders", e),
    };

    // Get all notes
    let notes = match state.db.get_notes(&user_id).await {
        Ok(notes) => notes
            .into_iter()
            .map(|n| NoteSync {
                id: n.id,
                title: n.title,
                content: n.content,
                folder_id: n.folder_id,
                position: n.position,
                is_important: n.is_important,
                created_at: n.created_at,
                updated_at: n.updated_at,
            })
            .collect(),
        Err(e) => return response::internal_error_logged("Failed to get notes", e),
    };

    // Get all board folders
    let board_folders = match state.db.get_board_folders(&user_id).await {
        Ok(folders) => folders
            .into_iter()
            .map(|f| BoardFolderSync {
                id: f.id,
                name: f.name,
                parent_id: f.parent_id,
                position: f.position,
                created_at: f.created_at,
                updated_at: f.updated_at,
            })
            .collect(),
        Err(e) => return response::internal_error_logged("Failed to get board folders", e),
    };

    // Get all boards with their lists, cards, and labels
    let boards_data = match state.db.get_boards(&user_id).await {
        Ok(b) => b,
        Err(e) => return response::internal_error_logged("Failed to get boards", e),
    };

    let mut boards = Vec::new();
    for board in boards_data {
        // Get lists for this board
        let lists_data = match state.db.get_lists(&board.id).await {
            Ok(l) => l,
            Err(e) => return response::internal_error_logged("Failed to get lists", e),
        };

        let mut lists = Vec::new();
        for list in lists_data {
            // Get cards for this list
            let cards_data = match state.db.get_cards(&list.id).await {
                Ok(c) => c,
                Err(e) => return response::internal_error_logged("Failed to get cards", e),
            };

            let cards: Vec<CardSync> = cards_data
                .into_iter()
                .map(|c| CardSync {
                    id: c.id,
                    title: c.title,
                    description: c.description,
                    position: c.position,
                    due_date: c.due_date,
                    archived: c.archived,
                    created_at: c.created_at,
                    updated_at: c.updated_at,
                    labels: c.labels,
                })
                .collect();

            lists.push(ListSync {
                id: list.id,
                name: list.name,
                position: list.position,
                archived: list.archived,
                created_at: list.created_at,
                updated_at: list.updated_at,
                cards,
            });
        }

        // Get labels for this board
        let labels_data = match state.db.get_labels(&board.id).await {
            Ok(l) => l,
            Err(e) => return response::internal_error_logged("Failed to get labels", e),
        };

        let labels: Vec<LabelSync> = labels_data
            .into_iter()
            .map(|l| LabelSync {
                id: l.id,
                name: l.name,
                color: l.color,
            })
            .collect();

        boards.push(BoardSync {
            id: board.id,
            name: board.name,
            description: board.description,
            folder_id: board.folder_id,
            position: board.position,
            created_at: board.created_at,
            updated_at: board.updated_at,
            lists,
            labels,
        });
    }

    let sync_response = FullSyncResponse {
        user,
        notes: NotesSync {
            folders: note_folders,
            notes,
        },
        boards: BoardsSync {
            folders: board_folders,
            boards,
        },
        synced_at: Utc::now(),
    };

    response::ok(sync_response)
}

/// Get note with full content for offline viewing
pub async fn get_note_full(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();

    let note = match state.db.get_note(&note_id, &user_id).await {
        Ok(n) => n,
        Err(crate::db::DbError::NotFound) => return response::not_found("Note"),
        Err(e) => return response::internal_error_logged("Failed to get note", e),
    };

    // Get attachments for this note
    let attachments = match state.db.get_note_attachments(&note_id).await {
        Ok(a) => a,
        Err(e) => return response::internal_error_logged("Failed to get attachments", e),
    };

    #[derive(Serialize)]
    struct NoteFullResponse {
        note: NoteSync,
        attachments: Vec<AttachmentInfo>,
    }

    #[derive(Serialize)]
    struct AttachmentInfo {
        id: String,
        filename: String,
        original_filename: String,
        mime_type: String,
        size: i64,
    }

    let note_response = NoteFullResponse {
        note: NoteSync {
            id: note.id,
            title: note.title,
            content: note.content,
            folder_id: note.folder_id,
            position: note.position,
            is_important: note.is_important,
            created_at: note.created_at,
            updated_at: note.updated_at,
        },
        attachments: attachments
            .into_iter()
            .map(|a| AttachmentInfo {
                id: a.id,
                filename: a.filename,
                original_filename: a.original_filename,
                mime_type: a.mime_type,
                size: a.size,
            })
            .collect(),
    };

    response::ok(note_response)
}

/// Get board with full content for offline viewing
pub async fn get_board_full(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    let board = match state.db.get_board(&board_id, &user_id).await {
        Ok(b) => b,
        Err(crate::db::DbError::NotFound) => return response::not_found("Board"),
        Err(e) => return response::internal_error_logged("Failed to get board", e),
    };

    // Get lists for this board
    let lists_data = match state.db.get_lists(&board_id).await {
        Ok(l) => l,
        Err(e) => return response::internal_error_logged("Failed to get lists", e),
    };

    let mut lists = Vec::new();
    for list in lists_data {
        // Get cards for this list
        let cards_data = match state.db.get_cards(&list.id).await {
            Ok(c) => c,
            Err(e) => return response::internal_error_logged("Failed to get cards", e),
        };

        let cards: Vec<CardSync> = cards_data
            .into_iter()
            .map(|c| CardSync {
                id: c.id,
                title: c.title,
                description: c.description,
                position: c.position,
                due_date: c.due_date,
                archived: c.archived,
                created_at: c.created_at,
                updated_at: c.updated_at,
                labels: c.labels,
            })
            .collect();

        lists.push(ListSync {
            id: list.id,
            name: list.name,
            position: list.position,
            archived: list.archived,
            created_at: list.created_at,
            updated_at: list.updated_at,
            cards,
        });
    }

    // Get labels for this board
    let labels_data = match state.db.get_labels(&board_id).await {
        Ok(l) => l,
        Err(e) => return response::internal_error_logged("Failed to get labels", e),
    };

    let labels: Vec<LabelSync> = labels_data
        .into_iter()
        .map(|l| LabelSync {
            id: l.id,
            name: l.name,
            color: l.color,
        })
        .collect();

    let board_response = BoardSync {
        id: board.id,
        name: board.name,
        description: board.description,
        folder_id: board.folder_id,
        position: board.position,
        created_at: board.created_at,
        updated_at: board.updated_at,
        lists,
        labels,
    };

    response::ok(board_response)
}
