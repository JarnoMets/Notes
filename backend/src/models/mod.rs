//! Models module

pub mod automation;
pub mod board;
pub mod card;
pub mod graph;
pub mod label;
pub mod list;
pub mod note;
pub mod note_revision;
pub mod reminder;
pub mod settings;
pub mod user;

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub use board::BoardsTree;
pub use note::NotesTree;
pub use graph::GraphsTree;

// Re-export all model structs for convenience
pub use user::{User, UserResponse, RegisterRequest, LoginRequest, GoogleAuthRequest, AuthResponse, Claims};
pub use note::{Note, NoteAttachment, NoteFolder, NoteWithAttachments, CreateNoteRequest, UpdateNoteRequest, MoveNoteRequest, CreateFolderRequest, UpdateFolderRequest, MoveFolderRequest};
pub use note_revision::NoteRevision;
pub use board::{Board, BoardFolder, BoardWithLists, CreateBoardRequest, UpdateBoardRequest, MoveBoardRequest, CreateBoardFolderRequest, UpdateBoardFolderRequest, MoveBoardFolderRequest};
pub use list::{List, ListWithCards, CreateListRequest, UpdateListRequest, ReorderListsRequest};
pub use card::{Card, CardAttachment, CardWithAttachments, CreateCardRequest, UpdateCardRequest, MoveCardRequest, ReorderCardsRequest};
pub use label::{BoardLabel, CreateLabelRequest, UpdateLabelRequest};
pub use automation::{AutomationRule, CreateAutomationRequest, UpdateAutomationRequest};
pub use reminder::{Reminder, CreateReminderRequest, UpdateReminderRequest};
pub use settings::{IcsCalendar, UserSettings, UpdateSettingsRequest, SettingsResponse};
pub use graph::{Graph, GraphFolder, GraphNode, GraphEdge, GraphWithData, CreateGraphRequest, UpdateGraphRequest, MoveGraphRequest, CreateGraphFolderRequest, UpdateGraphFolderRequest, MoveGraphFolderRequest, CreateNodeRequest, UpdateNodeRequest, CreateEdgeRequest, UpdateEdgeRequest};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::Database>,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub app_url: String,
    pub cache: Arc<Cache>,
}

#[derive(Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct Cache {
    notes_tree: Arc<RwLock<HashMap<String, CacheEntry<NotesTree>>>>,
    boards_tree: Arc<RwLock<HashMap<String, CacheEntry<BoardsTree>>>>,
    cache_duration: Duration,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            notes_tree: Arc::new(RwLock::new(HashMap::new())),
            boards_tree: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(30), // Cache for 30 seconds
        }
    }

    pub async fn get_notes_tree(&self, user_id: &str) -> Option<NotesTree> {
        let cache = self.notes_tree.read().await;
        if let Some(entry) = cache.get(user_id) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    pub async fn set_notes_tree(&self, user_id: String, tree: NotesTree) {
        let mut cache = self.notes_tree.write().await;
        let entry = CacheEntry {
            data: tree,
            expires_at: Instant::now() + self.cache_duration,
        };
        cache.insert(user_id, entry);
    }

    pub async fn invalidate_notes_tree(&self, user_id: &str) {
        let mut cache = self.notes_tree.write().await;
        cache.remove(user_id);
    }

    pub async fn get_boards_tree(&self, user_id: &str) -> Option<BoardsTree> {
        let cache = self.boards_tree.read().await;
        if let Some(entry) = cache.get(user_id) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    pub async fn set_boards_tree(&self, user_id: String, tree: BoardsTree) {
        let mut cache = self.boards_tree.write().await;
        let entry = CacheEntry {
            data: tree,
            expires_at: Instant::now() + self.cache_duration,
        };
        cache.insert(user_id, entry);
    }

    pub async fn invalidate_boards_tree(&self, user_id: &str) {
        let mut cache = self.boards_tree.write().await;
        cache.remove(user_id);
    }
}
