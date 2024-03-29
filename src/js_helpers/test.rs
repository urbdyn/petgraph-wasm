//! Test helpers for the library, such as generating test data.
//! Only compiled when building tests.

#![cfg(test)]

use crate::graph_impl::DiGraph;
use wasm_bindgen::JsValue;

/// Generate a test graph of 5 nodes with city names with 9 un-weighted edges
/// and no cycles.
pub fn new_test_graph() -> (DiGraph, Vec<usize>, Vec<usize>) {
    let mut g = DiGraph::new(None, None);
    let nodes = vec![
        g.add_node(JsValue::from_str("NYC")),
        g.add_node(JsValue::from_str("Vilnius")),
        g.add_node(JsValue::from_str("Knoxville")),
        g.add_node(JsValue::from_str("Taipei")),
        g.add_node(JsValue::from_str("Buenos Aires")),
    ];
    let edges = vec![
        g.add_edge(nodes[1], nodes[0], JsValue::NULL),
        g.add_edge(nodes[1], nodes[3], JsValue::NULL),
        g.add_edge(nodes[1], nodes[2], JsValue::NULL),
        g.add_edge(nodes[0], nodes[3], JsValue::NULL),
        g.add_edge(nodes[0], nodes[2], JsValue::NULL),
        g.add_edge(nodes[3], nodes[2], JsValue::NULL),
        g.add_edge(nodes[4], nodes[1], JsValue::NULL),
        g.add_edge(nodes[4], nodes[0], JsValue::NULL),
        g.add_edge(nodes[4], nodes[3], JsValue::NULL),
    ];
    (g, nodes, edges)
}
