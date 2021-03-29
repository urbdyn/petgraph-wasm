pub mod algo;
pub mod graph_impl;
pub mod js_helpers;
mod utils;

use petgraph::algo::Cycle;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphItemType {
    Node,
    Edge,
}
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphItem {
    pub component_type: GraphItemType,
    pub id: u32,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphError {
    message: String,
    component: GraphItem,
}

impl GraphError {
    pub fn new(message: &str, t: GraphItemType, i: u32) -> Self {
        GraphError {
            message: String::from(message),
            component: GraphItem {
                component_type: t,
                id: i,
            },
        }
    }

    pub fn create_js_value(&self) -> Result<JsValue, String> {
        JsValue::from_serde(&self).map_err(|_| String::from("Failed to transform graph_error"))
    }
}

impl From<Cycle<NodeIndex<u32>>> for GraphError {
    fn from(cycle: Cycle<NodeIndex<u32>>) -> Self {
        let cycle_node_id = cycle.node_id().index() as u32;
        GraphError::new("Cycle detected", GraphItemType::Node, cycle_node_id)
    }
}

impl From<NodeIndex> for GraphItem {
    fn from(index: NodeIndex<u32>) -> Self {
        GraphItem {
            component_type: GraphItemType::Node,
            id: index.index() as u32,
        }
    }
}
