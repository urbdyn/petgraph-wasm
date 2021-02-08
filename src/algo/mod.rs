use crate::graph_impl::DiGraph;
use crate::GraphError;
use petgraph::algo;
use petgraph::graph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn toposort(graph: &DiGraph) -> Result<Vec<usize>, JsValue> {
    match algo::toposort(&graph.graph, None) {
        Ok(sorted_nodes) => {
            let sorted_node_ids = sorted_nodes
                .into_iter()
                .map(|node: graph::NodeIndex| node.index())
                .collect();
            Result::Ok(sorted_node_ids)
        }
        Err(cycle) => match GraphError::from(cycle).create_js_value() {
            Ok(graph_error_js) => Result::Err(graph_error_js),
            Err(failure_msg) => Result::Err(JsValue::from_str(&failure_msg)),
        },
    }
}
