use super::{Database, DbError, DbResult};
use crate::models::{Graph, GraphEdge, GraphFolder, GraphNode, GraphsTree, GraphWithData};
use chrono::Utc;

impl Database {
    // ============ Graph Folders ============

    pub async fn get_graph_folders(&self, user_id: &str) -> DbResult<Vec<GraphFolder>> {
        let rows = sqlx::query_as::<_, GraphFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM graph_folders WHERE user_id = $1 ORDER BY position ASC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_graph_folder(&self, id: &str, user_id: &str) -> DbResult<GraphFolder> {
        let folder = sqlx::query_as::<_, GraphFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM graph_folders WHERE id = $1 AND user_id = $2"#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(folder)
    }

    pub async fn create_graph_folder(&self, folder: &GraphFolder) -> DbResult<GraphFolder> {
        sqlx::query(
            "INSERT INTO graph_folders (id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&folder.id)
        .bind(&folder.user_id)
        .bind(&folder.parent_id)
        .bind(&folder.name)
        .bind(&folder.position)
        .bind(&folder.is_important)
        .bind(&folder.is_urgent)
        .bind(&folder.created_at)
        .bind(&folder.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(folder.clone())
    }

    pub async fn update_graph_folder(
        &self,
        id: &str,
        user_id: &str,
        name: Option<String>,
        parent_id: Option<Option<String>>,
        position: Option<i32>,
        is_important: Option<bool>,
        is_urgent: Option<bool>,
    ) -> DbResult<GraphFolder> {
        let existing = self.get_graph_folder(id, user_id).await?;

        let new_name = name.unwrap_or(existing.name.clone());
        let new_parent_id = parent_id.clone().unwrap_or(existing.parent_id.clone());
        let new_position = position.unwrap_or(existing.position);
        let new_is_important = is_important.unwrap_or(existing.is_important);
        let new_is_urgent = is_urgent.unwrap_or(existing.is_urgent);
        let updated_at = Utc::now();

        // If position or parent is changing, we need to reorder
        if position.is_some() || parent_id.is_some() {
            let old_parent = existing.parent_id.clone();
            let target_parent = new_parent_id.clone();

            // First, shift items in the target parent to make room at new_position
            match target_parent.as_ref() {
                Some(pid) => {
                    sqlx::query(
                        "UPDATE graph_folders SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND parent_id = $3 AND position >= $4 AND id != $5",
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(pid)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
                None => {
                    sqlx::query(
                        "UPDATE graph_folders SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND parent_id IS NULL AND position >= $3 AND id != $4",
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
            }

            // If moving from a different parent, compact the old parent
            if old_parent != target_parent {
                match old_parent.as_ref() {
                    Some(pid) => {
                        sqlx::query(
                            "UPDATE graph_folders SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND parent_id = $3 AND position > $4",
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(pid)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                    None => {
                        sqlx::query(
                            "UPDATE graph_folders SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND parent_id IS NULL AND position > $3",
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                }
            }
        }

        sqlx::query(
            "UPDATE graph_folders SET name = $1, parent_id = $2, position = $3, is_important = $4, is_urgent = $5, updated_at = $6 WHERE id = $7 AND user_id = $8",
        )
        .bind(&new_name)
        .bind(&new_parent_id)
        .bind(&new_position)
        .bind(&new_is_important)
        .bind(&new_is_urgent)
        .bind(&updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_graph_folder(id, user_id).await
    }

    pub async fn delete_graph_folder(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM graph_folders WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn get_max_graph_folder_position(
        &self,
        user_id: &str,
        parent_id: Option<&str>,
    ) -> DbResult<i32> {
        let max: Option<i32> = match parent_id {
            Some(pid) => {
                sqlx::query_scalar(
                    "SELECT MAX(position) FROM graph_folders WHERE user_id = $1 AND parent_id = $2",
                )
                .bind(user_id)
                .bind(pid)
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar(
                    "SELECT MAX(position) FROM graph_folders WHERE user_id = $1 AND parent_id IS NULL",
                )
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    // ============ Graphs ============

    pub async fn get_graphs(&self, user_id: &str) -> DbResult<Vec<Graph>> {
        let rows = sqlx::query_as::<_, Graph>(
            r#"SELECT id, user_id, folder_id, name, description, position, is_important, is_urgent, created_at, updated_at 
               FROM graphs WHERE user_id = $1 ORDER BY position ASC"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_graphs_tree(&self, user_id: &str) -> DbResult<GraphsTree> {
        let folders = self.get_graph_folders(user_id).await?;
        let graphs = self.get_graphs(user_id).await?;
        Ok(GraphsTree { folders, graphs })
    }

    pub async fn get_graph(&self, id: &str, user_id: &str) -> DbResult<Graph> {
        let graph = sqlx::query_as::<_, Graph>(
            r#"SELECT id, user_id, folder_id, name, description, position, is_important, is_urgent, created_at, updated_at 
               FROM graphs WHERE id = $1 AND user_id = $2"#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(graph)
    }

    pub async fn get_graph_with_data(&self, id: &str, user_id: &str) -> DbResult<GraphWithData> {
        let graph = self.get_graph(id, user_id).await?;
        let nodes = self.get_graph_nodes(&graph.id).await?;
        let edges = self.get_graph_edges(&graph.id).await?;

        Ok(GraphWithData {
            graph,
            nodes,
            edges,
        })
    }

    pub async fn create_graph(&self, graph: &Graph) -> DbResult<Graph> {
        sqlx::query(
            "INSERT INTO graphs (id, user_id, folder_id, name, description, position, is_important, is_urgent, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(&graph.id)
        .bind(&graph.user_id)
        .bind(&graph.folder_id)
        .bind(&graph.name)
        .bind(&graph.description)
        .bind(&graph.position)
        .bind(&graph.is_important)
        .bind(&graph.is_urgent)
        .bind(&graph.created_at)
        .bind(&graph.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(graph.clone())
    }

    pub async fn update_graph(
        &self,
        id: &str,
        user_id: &str,
        name: Option<String>,
        description: Option<String>,
        folder_id: Option<Option<String>>,
        position: Option<i32>,
        is_important: Option<bool>,
        is_urgent: Option<bool>,
    ) -> DbResult<Graph> {
        let existing = self.get_graph(id, user_id).await?;

        let new_name = name.unwrap_or(existing.name.clone());
        let new_description = description.or(existing.description.clone());
        let new_folder_id = folder_id.clone().unwrap_or(existing.folder_id.clone());
        let new_position = position.unwrap_or(existing.position);
        let new_is_important = is_important.unwrap_or(existing.is_important);
        let new_is_urgent = is_urgent.unwrap_or(existing.is_urgent);
        let updated_at = Utc::now();

        // If position or folder is changing, we need to reorder
        if position.is_some() || folder_id.is_some() {
            let old_folder = existing.folder_id.clone();
            let target_folder = new_folder_id.clone();

            // First, shift items in the target folder to make room at new_position
            match target_folder.as_ref() {
                Some(fid) => {
                    sqlx::query(
                        "UPDATE graphs SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND folder_id = $3 AND position >= $4 AND id != $5",
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(fid)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
                None => {
                    sqlx::query(
                        "UPDATE graphs SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND folder_id IS NULL AND position >= $3 AND id != $4",
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
            }

            // If moving from a different folder, compact the old folder
            if old_folder != target_folder {
                match old_folder.as_ref() {
                    Some(fid) => {
                        sqlx::query(
                            "UPDATE graphs SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND folder_id = $3 AND position > $4",
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(fid)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                    None => {
                        sqlx::query(
                            "UPDATE graphs SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND folder_id IS NULL AND position > $3",
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                }
            }
        }

        sqlx::query(
            "UPDATE graphs SET name = $1, description = $2, folder_id = $3, position = $4, is_important = $5, is_urgent = $6, updated_at = $7 WHERE id = $8 AND user_id = $9",
        )
        .bind(&new_name)
        .bind(&new_description)
        .bind(&new_folder_id)
        .bind(&new_position)
        .bind(&new_is_important)
        .bind(&new_is_urgent)
        .bind(&updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_graph(id, user_id).await
    }

    pub async fn get_max_graph_position(
        &self,
        user_id: &str,
        folder_id: Option<&str>,
    ) -> DbResult<i32> {
        let max: Option<i32> = match folder_id {
            Some(fid) => {
                sqlx::query_scalar(
                    "SELECT MAX(position) FROM graphs WHERE user_id = $1 AND folder_id = $2",
                )
                .bind(user_id)
                .bind(fid)
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar(
                    "SELECT MAX(position) FROM graphs WHERE user_id = $1 AND folder_id IS NULL",
                )
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    pub async fn delete_graph(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM graphs WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    // ============ Graph Nodes ============

    pub async fn get_graph_nodes(&self, graph_id: &str) -> DbResult<Vec<GraphNode>> {
        let rows = sqlx::query_as::<_, GraphNode>(
            r#"SELECT id, graph_id, node_type, shape, label, reference_id, x, y, width, height, 
                      color, border_color, text_color, font_size, metadata, created_at, updated_at 
               FROM graph_nodes WHERE graph_id = $1 ORDER BY created_at ASC"#,
        )
        .bind(graph_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_graph_node(&self, id: &str, graph_id: &str) -> DbResult<GraphNode> {
        let node = sqlx::query_as::<_, GraphNode>(
            r#"SELECT id, graph_id, node_type, shape, label, reference_id, x, y, width, height, 
                      color, border_color, text_color, font_size, metadata, created_at, updated_at 
               FROM graph_nodes WHERE id = $1 AND graph_id = $2"#,
        )
        .bind(id)
        .bind(graph_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(node)
    }

    pub async fn create_graph_node(&self, node: &GraphNode) -> DbResult<GraphNode> {
        sqlx::query(
            "INSERT INTO graph_nodes (id, graph_id, node_type, shape, label, reference_id, x, y, width, height, 
                                      color, border_color, text_color, font_size, metadata, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)",
        )
        .bind(&node.id)
        .bind(&node.graph_id)
        .bind(&node.node_type)
        .bind(&node.shape)
        .bind(&node.label)
        .bind(&node.reference_id)
        .bind(&node.x)
        .bind(&node.y)
        .bind(&node.width)
        .bind(&node.height)
        .bind(&node.color)
        .bind(&node.border_color)
        .bind(&node.text_color)
        .bind(&node.font_size)
        .bind(&node.metadata)
        .bind(&node.created_at)
        .bind(&node.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(node.clone())
    }

    pub async fn update_graph_node(
        &self,
        id: &str,
        graph_id: &str,
        node_type: Option<String>,
        shape: Option<String>,
        label: Option<String>,
        reference_id: Option<Option<String>>,
        x: Option<f64>,
        y: Option<f64>,
        width: Option<f64>,
        height: Option<f64>,
        color: Option<Option<String>>,
        border_color: Option<Option<String>>,
        text_color: Option<Option<String>>,
        font_size: Option<Option<i32>>,
        metadata: Option<Option<String>>,
    ) -> DbResult<GraphNode> {
        let existing = self.get_graph_node(id, graph_id).await?;

        let new_node_type = node_type.unwrap_or(existing.node_type.clone());
        let new_shape = shape.unwrap_or(existing.shape.clone());
        let new_label = label.unwrap_or(existing.label.clone());
        let new_reference_id = reference_id.unwrap_or(existing.reference_id.clone());
        let new_x = x.unwrap_or(existing.x);
        let new_y = y.unwrap_or(existing.y);
        let new_width = width.unwrap_or(existing.width);
        let new_height = height.unwrap_or(existing.height);
        let new_color = color.unwrap_or(existing.color.clone());
        let new_border_color = border_color.unwrap_or(existing.border_color.clone());
        let new_text_color = text_color.unwrap_or(existing.text_color.clone());
        let new_font_size = font_size.unwrap_or(existing.font_size);
        let new_metadata = metadata.unwrap_or(existing.metadata.clone());
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE graph_nodes SET node_type = $1, shape = $2, label = $3, reference_id = $4, 
             x = $5, y = $6, width = $7, height = $8, color = $9, border_color = $10, 
             text_color = $11, font_size = $12, metadata = $13, updated_at = $14 
             WHERE id = $15 AND graph_id = $16",
        )
        .bind(&new_node_type)
        .bind(&new_shape)
        .bind(&new_label)
        .bind(&new_reference_id)
        .bind(&new_x)
        .bind(&new_y)
        .bind(&new_width)
        .bind(&new_height)
        .bind(&new_color)
        .bind(&new_border_color)
        .bind(&new_text_color)
        .bind(&new_font_size)
        .bind(&new_metadata)
        .bind(&updated_at)
        .bind(id)
        .bind(graph_id)
        .execute(&self.pool)
        .await?;

        self.get_graph_node(id, graph_id).await
    }

    pub async fn delete_graph_node(&self, id: &str, graph_id: &str) -> DbResult<()> {
        // First delete any edges connected to this node
        sqlx::query(
            "DELETE FROM graph_edges WHERE graph_id = $1 AND (source_node_id = $2 OR target_node_id = $2)",
        )
        .bind(graph_id)
        .bind(id)
        .execute(&self.pool)
        .await?;

        let result = sqlx::query("DELETE FROM graph_nodes WHERE id = $1 AND graph_id = $2")
            .bind(id)
            .bind(graph_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    // ============ Graph Edges ============

    pub async fn get_graph_edges(&self, graph_id: &str) -> DbResult<Vec<GraphEdge>> {
        let rows = sqlx::query_as::<_, GraphEdge>(
            r#"SELECT id, graph_id, source_node_id, target_node_id, edge_type, style, 
                      label, color, thickness, metadata, created_at, updated_at 
               FROM graph_edges WHERE graph_id = $1 ORDER BY created_at ASC"#,
        )
        .bind(graph_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_graph_edge(&self, id: &str, graph_id: &str) -> DbResult<GraphEdge> {
        let edge = sqlx::query_as::<_, GraphEdge>(
            r#"SELECT id, graph_id, source_node_id, target_node_id, edge_type, style, 
                      label, color, thickness, metadata, created_at, updated_at 
               FROM graph_edges WHERE id = $1 AND graph_id = $2"#,
        )
        .bind(id)
        .bind(graph_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(edge)
    }

    pub async fn create_graph_edge(&self, edge: &GraphEdge) -> DbResult<GraphEdge> {
        sqlx::query(
            "INSERT INTO graph_edges (id, graph_id, source_node_id, target_node_id, edge_type, style, 
                                      label, color, thickness, metadata, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(&edge.id)
        .bind(&edge.graph_id)
        .bind(&edge.source_node_id)
        .bind(&edge.target_node_id)
        .bind(&edge.edge_type)
        .bind(&edge.style)
        .bind(&edge.label)
        .bind(&edge.color)
        .bind(&edge.thickness)
        .bind(&edge.metadata)
        .bind(&edge.created_at)
        .bind(&edge.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(edge.clone())
    }

    pub async fn update_graph_edge(
        &self,
        id: &str,
        graph_id: &str,
        source_node_id: Option<String>,
        target_node_id: Option<String>,
        edge_type: Option<String>,
        style: Option<String>,
        label: Option<Option<String>>,
        color: Option<Option<String>>,
        thickness: Option<Option<i32>>,
        metadata: Option<Option<String>>,
    ) -> DbResult<GraphEdge> {
        let existing = self.get_graph_edge(id, graph_id).await?;

        let new_source_node_id = source_node_id.unwrap_or(existing.source_node_id.clone());
        let new_target_node_id = target_node_id.unwrap_or(existing.target_node_id.clone());
        let new_edge_type = edge_type.unwrap_or(existing.edge_type.clone());
        let new_style = style.unwrap_or(existing.style.clone());
        let new_label = label.unwrap_or(existing.label.clone());
        let new_color = color.unwrap_or(existing.color.clone());
        let new_thickness = thickness.unwrap_or(existing.thickness);
        let new_metadata = metadata.unwrap_or(existing.metadata.clone());
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE graph_edges SET source_node_id = $1, target_node_id = $2, edge_type = $3, 
             style = $4, label = $5, color = $6, thickness = $7, metadata = $8, updated_at = $9 
             WHERE id = $10 AND graph_id = $11",
        )
        .bind(&new_source_node_id)
        .bind(&new_target_node_id)
        .bind(&new_edge_type)
        .bind(&new_style)
        .bind(&new_label)
        .bind(&new_color)
        .bind(&new_thickness)
        .bind(&new_metadata)
        .bind(&updated_at)
        .bind(id)
        .bind(graph_id)
        .execute(&self.pool)
        .await?;

        self.get_graph_edge(id, graph_id).await
    }

    pub async fn delete_graph_edge(&self, id: &str, graph_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM graph_edges WHERE id = $1 AND graph_id = $2")
            .bind(id)
            .bind(graph_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }
}
