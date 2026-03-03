//! Graph-related models for visual graph/network diagrams

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

/// Helper for deserializing Option<Option<T>> to distinguish between missing and null
fn deserialize_double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::<T>::deserialize(deserializer)?))
}

/// Graph folder for organizing graphs hierarchically
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GraphFolder {
    pub id: String,
    pub user_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub position: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GraphFolder {
    pub fn new(user_id: String, parent_id: Option<String>, name: String, position: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            parent_id,
            name,
            position,
            is_important: false,
            is_urgent: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a graph folder
#[derive(Debug, Deserialize)]
pub struct CreateGraphFolderRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

/// Request body for updating a graph folder
#[derive(Debug, Deserialize)]
pub struct UpdateGraphFolderRequest {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub parent_id: Option<Option<String>>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a graph folder
#[derive(Debug, Deserialize)]
pub struct MoveGraphFolderRequest {
    pub parent_id: Option<String>,
    pub position: i32,
}

/// Graph entity - the main canvas for drawing network diagrams
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Graph {
    pub id: String,
    pub user_id: String,
    pub folder_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Graph {
    pub fn new(
        user_id: String,
        folder_id: Option<String>,
        name: String,
        description: Option<String>,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            folder_id,
            name,
            description,
            position,
            is_important: false,
            is_urgent: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a graph
#[derive(Debug, Deserialize)]
pub struct CreateGraphRequest {
    pub name: String,
    pub description: Option<String>,
    pub folder_id: Option<String>,
}

/// Request body for updating a graph
#[derive(Debug, Deserialize)]
pub struct UpdateGraphRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub folder_id: Option<Option<String>>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a graph
#[derive(Debug, Deserialize)]
pub struct MoveGraphRequest {
    pub folder_id: Option<String>,
    pub position: i32,
}

/// Node type - what kind of node this is
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// A simple bubble/shape node with text
    Bubble,
    /// Reference to a Note
    Note,
    /// Reference to a Board
    Board,
    /// Reference to a Card in a Board
    Card,
    /// Reference to another Graph
    Graph,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Bubble
    }
}

/// Node shape for bubble nodes
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeShape {
    Circle,
    Rectangle,
    RoundedRect,
    Diamond,
    Hexagon,
    Ellipse,
}

impl Default for NodeShape {
    fn default() -> Self {
        NodeShape::RoundedRect
    }
}

/// A node in the graph (can be a bubble, note reference, board reference, or card reference)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GraphNode {
    pub id: String,
    pub graph_id: String,
    pub node_type: String, // Stored as string in DB, converted to/from NodeType
    pub shape: String,     // Stored as string in DB, converted to/from NodeShape
    pub label: String,
    /// Reference ID (note_id, board_id, card_id, or graph_id depending on node_type)
    pub reference_id: Option<String>,
    /// X position on canvas
    pub x: f64,
    /// Y position on canvas
    pub y: f64,
    /// Width of the node
    pub width: f64,
    /// Height of the node
    pub height: f64,
    /// Background color
    pub color: Option<String>,
    /// Border color
    pub border_color: Option<String>,
    /// Text color
    pub text_color: Option<String>,
    /// Font size
    pub font_size: Option<i32>,
    /// Additional metadata as JSON
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GraphNode {
    #[allow(dead_code)]
    pub fn new(
        graph_id: String,
        node_type: NodeType,
        shape: NodeShape,
        label: String,
        reference_id: Option<String>,
        x: f64,
        y: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            graph_id,
            node_type: serde_json::to_string(&node_type).unwrap_or_else(|_| "\"bubble\"".to_string()).trim_matches('"').to_string(),
            shape: serde_json::to_string(&shape).unwrap_or_else(|_| "\"rounded_rect\"".to_string()).trim_matches('"').to_string(),
            label,
            reference_id,
            x,
            y,
            width: 120.0,
            height: 60.0,
            color: None,
            border_color: None,
            text_color: None,
            font_size: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a node
#[derive(Debug, Deserialize)]
pub struct CreateNodeRequest {
    pub node_type: Option<String>,
    pub shape: Option<String>,
    pub label: String,
    pub reference_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub color: Option<String>,
    pub border_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<i32>,
    pub metadata: Option<String>,
}

/// Request body for updating a node
#[derive(Debug, Deserialize)]
pub struct UpdateNodeRequest {
    pub node_type: Option<String>,
    pub shape: Option<String>,
    pub label: Option<String>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub reference_id: Option<Option<String>>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub color: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub border_color: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub text_color: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub font_size: Option<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub metadata: Option<Option<String>>,
}

/// Edge/connection type
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    /// Simple line (no arrows)
    Line,
    /// Arrow pointing to target
    Arrow,
    /// Arrow pointing to source
    ReverseArrow,
    /// Arrows on both ends
    Bidirectional,
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::Arrow
    }
}

/// Edge line style
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeStyle {
    Solid,
    Dashed,
    Dotted,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        EdgeStyle::Solid
    }
}

/// An edge/connection between two nodes in the graph
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GraphEdge {
    pub id: String,
    pub graph_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String, // Stored as string in DB
    pub style: String,     // Stored as string in DB
    pub label: Option<String>,
    pub color: Option<String>,
    pub thickness: Option<i32>,
    /// Additional metadata as JSON
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GraphEdge {
    #[allow(dead_code)]
    pub fn new(
        graph_id: String,
        source_node_id: String,
        target_node_id: String,
        edge_type: EdgeType,
        style: EdgeStyle,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            graph_id,
            source_node_id,
            target_node_id,
            edge_type: serde_json::to_string(&edge_type).unwrap_or_else(|_| "\"arrow\"".to_string()).trim_matches('"').to_string(),
            style: serde_json::to_string(&style).unwrap_or_else(|_| "\"solid\"".to_string()).trim_matches('"').to_string(),
            label: None,
            color: None,
            thickness: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating an edge
#[derive(Debug, Deserialize)]
pub struct CreateEdgeRequest {
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: Option<String>,
    pub style: Option<String>,
    pub label: Option<String>,
    pub color: Option<String>,
    pub thickness: Option<i32>,
    pub metadata: Option<String>,
}

/// Request body for updating an edge
#[derive(Debug, Deserialize)]
pub struct UpdateEdgeRequest {
    pub source_node_id: Option<String>,
    pub target_node_id: Option<String>,
    pub edge_type: Option<String>,
    pub style: Option<String>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub color: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub thickness: Option<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub metadata: Option<Option<String>>,
}

/// Graph tree with folders and graphs
#[derive(Debug, Clone, Serialize)]
pub struct GraphsTree {
    pub folders: Vec<GraphFolder>,
    pub graphs: Vec<Graph>,
}

/// Graph with all its nodes and edges
#[derive(Debug, Clone, Serialize)]
pub struct GraphWithData {
    pub graph: Graph,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}
