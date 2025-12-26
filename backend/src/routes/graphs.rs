//! Graph route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{
    AppState, CreateEdgeRequest, CreateGraphFolderRequest, CreateGraphRequest, CreateNodeRequest,
    Graph, GraphEdge, GraphFolder, GraphNode, MoveGraphFolderRequest, MoveGraphRequest,
    UpdateEdgeRequest, UpdateGraphFolderRequest, UpdateGraphRequest, UpdateNodeRequest,
};
use crate::require_auth;

use super::response::{created, internal_error_logged, no_content, not_found, ok};

// ============ Graph Folders ============

pub async fn get_graph_folders(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_graph_folders(&user_id).await {
        Ok(folders) => ok(folders),
        Err(e) => internal_error_logged("Failed to get graph folders", e),
    }
}

pub async fn create_graph_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateGraphFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_graph_folder_position(&user_id, body.parent_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let folder = GraphFolder::new(user_id, body.parent_id.clone(), body.name.clone(), position);

    match state.db.create_graph_folder(&folder).await {
        Ok(folder) => created(folder),
        Err(e) => internal_error_logged("Failed to create graph folder", e),
    }
}

pub async fn update_graph_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateGraphFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let parent_id = body.parent_id.clone();

    match state
        .db
        .update_graph_folder(
            &id,
            &user_id,
            body.name.clone(),
            parent_id,
            body.position,
            body.is_important,
            body.is_urgent,
        )
        .await
    {
        Ok(folder) => ok(folder),
        Err(DbError::NotFound) => not_found("Graph folder"),
        Err(e) => internal_error_logged("Failed to update graph folder", e),
    }
}

pub async fn move_graph_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveGraphFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_graph_folder(
            &id,
            &user_id,
            None,
            Some(body.parent_id.clone()),
            Some(body.position),
            None,
            None,
        )
        .await
    {
        Ok(folder) => ok(folder),
        Err(DbError::NotFound) => not_found("Graph folder"),
        Err(e) => internal_error_logged("Failed to move graph folder", e),
    }
}

pub async fn delete_graph_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_graph_folder(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Graph folder"),
        Err(e) => internal_error_logged("Failed to delete graph folder", e),
    }
}

// ============ Graphs ============

pub async fn get_graphs(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_graphs(&user_id).await {
        Ok(graphs) => ok(graphs),
        Err(e) => internal_error_logged("Failed to get graphs", e),
    }
}

pub async fn get_graphs_tree(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_graphs_tree(&user_id).await {
        Ok(tree) => ok(tree),
        Err(DbError::NotFound) => not_found("Graphs tree"),
        Err(e) => internal_error_logged("Failed to get graphs tree", e),
    }
}

pub async fn get_graph(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.get_graph_with_data(&id, &user_id).await {
        Ok(graph) => ok(graph),
        Err(DbError::NotFound) => not_found("Graph"),
        Err(e) => internal_error_logged("Failed to get graph", e),
    }
}

pub async fn create_graph(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateGraphRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_graph_position(&user_id, body.folder_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let graph = Graph::new(
        user_id.clone(),
        body.folder_id.clone(),
        body.name.clone(),
        body.description.clone(),
        position,
    );

    match state.db.create_graph(&graph).await {
        Ok(graph) => created(graph),
        Err(e) => internal_error_logged("Failed to create graph", e),
    }
}

pub async fn update_graph(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateGraphRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let folder_id = body.folder_id.clone();

    match state
        .db
        .update_graph(
            &id,
            &user_id,
            body.name.clone(),
            body.description.clone(),
            folder_id,
            body.position,
            body.is_important,
            body.is_urgent,
        )
        .await
    {
        Ok(graph) => ok(graph),
        Err(DbError::NotFound) => not_found("Graph"),
        Err(e) => internal_error_logged("Failed to update graph", e),
    }
}

pub async fn move_graph(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveGraphRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_graph(
            &id,
            &user_id,
            None,
            None,
            Some(body.folder_id.clone()),
            Some(body.position),
            None,
            None,
        )
        .await
    {
        Ok(graph) => ok(graph),
        Err(DbError::NotFound) => not_found("Graph"),
        Err(e) => internal_error_logged("Failed to move graph", e),
    }
}

pub async fn delete_graph(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_graph(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Graph"),
        Err(e) => internal_error_logged("Failed to delete graph", e),
    }
}

// ============ Graph Nodes ============

pub async fn get_nodes(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let graph_id = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state.db.get_graph_nodes(&graph_id).await {
        Ok(nodes) => ok(nodes),
        Err(e) => internal_error_logged("Failed to get nodes", e),
    }
}

pub async fn create_node(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateNodeRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let graph_id = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    let now = chrono::Utc::now();
    let node = GraphNode {
        id: uuid::Uuid::new_v4().to_string(),
        graph_id,
        node_type: body.node_type.clone().unwrap_or_else(|| "bubble".to_string()),
        shape: body.shape.clone().unwrap_or_else(|| "rounded_rect".to_string()),
        label: body.label.clone(),
        reference_id: body.reference_id.clone(),
        x: body.x,
        y: body.y,
        width: body.width.unwrap_or(120.0),
        height: body.height.unwrap_or(60.0),
        color: body.color.clone(),
        border_color: body.border_color.clone(),
        text_color: body.text_color.clone(),
        font_size: body.font_size,
        metadata: body.metadata.clone(),
        created_at: now,
        updated_at: now,
    };

    match state.db.create_graph_node(&node).await {
        Ok(node) => created(node),
        Err(e) => internal_error_logged("Failed to create node", e),
    }
}

pub async fn update_node(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Json<UpdateNodeRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let (graph_id, node_id) = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state
        .db
        .update_graph_node(
            &node_id,
            &graph_id,
            body.node_type.clone(),
            body.shape.clone(),
            body.label.clone(),
            body.reference_id.clone(),
            body.x,
            body.y,
            body.width,
            body.height,
            body.color.clone(),
            body.border_color.clone(),
            body.text_color.clone(),
            body.font_size,
            body.metadata.clone(),
        )
        .await
    {
        Ok(node) => ok(node),
        Err(DbError::NotFound) => not_found("Node"),
        Err(e) => internal_error_logged("Failed to update node", e),
    }
}

pub async fn delete_node(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let (graph_id, node_id) = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state.db.delete_graph_node(&node_id, &graph_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Node"),
        Err(e) => internal_error_logged("Failed to delete node", e),
    }
}

// ============ Graph Edges ============

pub async fn get_edges(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let graph_id = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state.db.get_graph_edges(&graph_id).await {
        Ok(edges) => ok(edges),
        Err(e) => internal_error_logged("Failed to get edges", e),
    }
}

pub async fn create_edge(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateEdgeRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let graph_id = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    let now = chrono::Utc::now();
    let edge = GraphEdge {
        id: uuid::Uuid::new_v4().to_string(),
        graph_id,
        source_node_id: body.source_node_id.clone(),
        target_node_id: body.target_node_id.clone(),
        edge_type: body.edge_type.clone().unwrap_or_else(|| "arrow".to_string()),
        style: body.style.clone().unwrap_or_else(|| "solid".to_string()),
        label: body.label.clone(),
        color: body.color.clone(),
        thickness: body.thickness,
        metadata: body.metadata.clone(),
        created_at: now,
        updated_at: now,
    };

    match state.db.create_graph_edge(&edge).await {
        Ok(edge) => created(edge),
        Err(e) => internal_error_logged("Failed to create edge", e),
    }
}

pub async fn update_edge(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Json<UpdateEdgeRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let (graph_id, edge_id) = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state
        .db
        .update_graph_edge(
            &edge_id,
            &graph_id,
            body.source_node_id.clone(),
            body.target_node_id.clone(),
            body.edge_type.clone(),
            body.style.clone(),
            body.label.clone(),
            body.color.clone(),
            body.thickness,
            body.metadata.clone(),
        )
        .await
    {
        Ok(edge) => ok(edge),
        Err(DbError::NotFound) => not_found("Edge"),
        Err(e) => internal_error_logged("Failed to update edge", e),
    }
}

pub async fn delete_edge(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let (graph_id, edge_id) = path.into_inner();

    // Verify user owns this graph
    if let Err(e) = state.db.get_graph(&graph_id, &user_id).await {
        return match e {
            DbError::NotFound => not_found("Graph"),
            e => internal_error_logged("Failed to get graph", e),
        };
    }

    match state.db.delete_graph_edge(&edge_id, &graph_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Edge"),
        Err(e) => internal_error_logged("Failed to delete edge", e),
    }
}
