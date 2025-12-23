use actix_web::web;

use crate::routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(routes::health_check))
            // Auth routes
            .route("/auth/register", web::post().to(routes::auth::register))
            .route("/auth/login", web::post().to(routes::auth::login))
            .route("/auth/google", web::post().to(routes::auth::google_auth))
            .route("/auth/google/mobile", web::post().to(routes::auth::google_auth_mobile))
            .route("/auth/google/url", web::get().to(routes::auth::get_google_auth_url))
            .route("/auth/me", web::get().to(routes::auth::get_me))
            .route("/auth/storage", web::get().to(routes::auth::get_storage))
            // Notes routes
            .route("/notes", web::get().to(routes::notes::get_notes))
            .route("/notes", web::post().to(routes::notes::create_note))
            .route("/notes/tree", web::get().to(routes::notes::get_notes_tree))
            .route("/notes/{id}", web::get().to(routes::notes::get_note))
            .route("/notes/{id}", web::put().to(routes::notes::update_note))
            .route("/notes/{id}", web::delete().to(routes::notes::delete_note))
            .route("/notes/{id}/move", web::post().to(routes::notes::move_note))
            .route("/notes/{id}/revisions", web::get().to(routes::notes::list_revisions))
            .route("/notes/{id}/revisions/undo", web::post().to(routes::notes::undo_revision))
            .route("/notes/{id}/revisions/redo", web::post().to(routes::notes::redo_revision))
            // Note folders routes
            .route("/folders", web::get().to(routes::folders::get_folders))
            .route("/folders", web::post().to(routes::folders::create_folder))
            .route("/folders/{id}", web::put().to(routes::folders::update_folder))
            .route("/folders/{id}", web::delete().to(routes::folders::delete_folder))
            .route("/folders/{id}/move", web::post().to(routes::folders::move_folder))
            // Note attachments routes
            .route("/notes/{id}/attachments", web::post().to(routes::attachments::upload_attachment))
            .route("/attachments/{id}", web::get().to(routes::attachments::download_attachment))
            .route("/attachments/{id}", web::delete().to(routes::attachments::delete_attachment))
            // Boards routes
            .route("/boards", web::get().to(routes::boards::get_boards))
            .route("/boards", web::post().to(routes::boards::create_board))
            .route("/boards/tree", web::get().to(routes::boards::get_boards_tree))
            .route("/boards/{id}", web::get().to(routes::boards::get_board))
            .route("/boards/{id}", web::put().to(routes::boards::update_board))
            .route("/boards/{id}", web::delete().to(routes::boards::delete_board))
            .route("/boards/{id}/move", web::post().to(routes::boards::move_board))
            // Board folders routes
            .route("/board-folders", web::get().to(routes::boards::get_board_folders))
            .route("/board-folders", web::post().to(routes::boards::create_board_folder))
            .route("/board-folders/{id}", web::put().to(routes::boards::update_board_folder))
            .route("/board-folders/{id}", web::delete().to(routes::boards::delete_board_folder))
            .route("/board-folders/{id}/move", web::post().to(routes::boards::move_board_folder))
            // Lists routes
            .route("/boards/{board_id}/lists", web::get().to(routes::lists::get_lists))
            .route("/boards/{board_id}/lists", web::post().to(routes::lists::create_list))
            .route("/lists/{id}", web::put().to(routes::lists::update_list))
            .route("/lists/{id}", web::delete().to(routes::lists::delete_list))
            .route("/lists/reorder", web::post().to(routes::lists::reorder_lists))
            // Cards routes
            .route("/lists/{list_id}/cards", web::get().to(routes::cards::get_cards))
            .route("/lists/{list_id}/cards", web::post().to(routes::cards::create_card))
            .route("/cards/{id}", web::get().to(routes::cards::get_card))
            .route("/cards/{id}", web::put().to(routes::cards::update_card))
            .route("/cards/{id}", web::delete().to(routes::cards::delete_card))
            .route("/cards/move", web::post().to(routes::cards::move_card))
            .route("/cards/reorder", web::post().to(routes::cards::reorder_cards))
            .route("/cards/{id}/archive", web::post().to(routes::cards::archive_card))
            .route("/cards/{id}/restore", web::post().to(routes::cards::restore_card))
            .route("/boards/{board_id}/cards/archived", web::get().to(routes::cards::get_archived_cards))
            // Labels routes
            .route("/boards/{board_id}/labels", web::get().to(routes::labels::get_labels))
            .route("/boards/{board_id}/labels", web::post().to(routes::labels::create_label))
            .route("/labels/{id}", web::put().to(routes::labels::update_label))
            .route("/labels/{id}", web::delete().to(routes::labels::delete_label))
            // Automations routes
            .route("/boards/{board_id}/automations", web::get().to(routes::automations::get_automations))
            .route("/boards/{board_id}/automations", web::post().to(routes::automations::create_automation))
            .route("/automations/{id}", web::put().to(routes::automations::update_automation))
            .route("/automations/{id}", web::delete().to(routes::automations::delete_automation))
            .route("/automations/{id}/toggle", web::post().to(routes::automations::toggle_automation))
            // Lists archive routes
            .route("/lists/{id}/archive", web::post().to(routes::lists::archive_list))
            .route("/lists/{id}/restore", web::post().to(routes::lists::restore_list))
            .route("/boards/{board_id}/lists/archived", web::get().to(routes::lists::get_archived_lists))
            // Sync routes for mobile app
            .route("/sync", web::get().to(routes::sync::full_sync))
            .route("/sync/notes/{id}", web::get().to(routes::sync::get_note_full))
            .route("/sync/boards/{id}", web::get().to(routes::sync::get_board_full))
            // Settings routes
            .route("/settings", web::get().to(routes::settings::get_settings))
            .route("/settings", web::put().to(routes::settings::update_settings))
            // Reminders routes
            .route("/reminders", web::get().to(routes::reminders::get_reminders))
            .route("/reminders", web::post().to(routes::reminders::create_reminder))
            .route("/reminders/{id}", web::get().to(routes::reminders::get_reminder))
            .route("/reminders/{id}", web::put().to(routes::reminders::update_reminder))
            .route("/reminders/{id}", web::delete().to(routes::reminders::delete_reminder))
            .route("/notes/{id}/reminders", web::delete().to(routes::reminders::delete_reminders_by_note)),
    );
}