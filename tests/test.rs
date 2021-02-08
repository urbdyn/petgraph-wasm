//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate petgraph_wasm;
extern crate wasm_bindgen_test;
extern crate web_sys;
use petgraph_wasm::algo::toposort;
use petgraph_wasm::graph_impl::DiGraph;
use petgraph_wasm::{GraphError, GraphItemType};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

//macro_rules! log {
//    ( $( $t:tt )* ) => {
//        web_sys::console::log_1(&format!( $( $t )* ).into());
//    }
//}

//wasm_bindgen_test_configure!(run_in_browser);

fn new_test_graph() -> (DiGraph, Vec<usize>, Vec<usize>) {
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
    return (g, nodes, edges);
}

#[wasm_bindgen_test]
fn can_create_graph() {
    let (g, nodes, edges) = new_test_graph();
    assert_eq!(g.node_count(), 5);
    assert_eq!(g.edge_count(), 9);
    assert_eq!(nodes, vec![0, 1, 2, 3, 4]);
    assert_eq!(edges, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[wasm_bindgen_test]
fn can_get_nodes_by_index() {
    let (g, _nodes, _edges) = new_test_graph();
    let nyc_name = g.node_weight(0).unwrap();
    let vilnius_name = g.node_weight(1).unwrap();
    let na_name = g.node_weight(9999).unwrap_err();
    assert_eq!(nyc_name, "NYC");
    assert_eq!(vilnius_name, "Vilnius");
    assert_eq!(na_name, JsValue::from_str("No node exists for given index"));
}

#[wasm_bindgen_test]
fn can_sort_nodes() {
    let (g, _nodes, _edges) = new_test_graph();
    let sorted_nodes = toposort(&g).unwrap();
    assert_eq!(sorted_nodes, vec![4, 1, 0, 3, 2]);
}

#[wasm_bindgen_test]
fn can_detect_cycles() {
    let (mut g, _nodes, _edges) = new_test_graph();
    g.add_edge(2, 1, JsValue::NULL);
    let sort_err = toposort(&g).unwrap_err();
    let expect_err = GraphError::new("Cycle detected", GraphItemType::Node, 1);
    assert_eq!(sort_err.into_serde::<GraphError>().unwrap(), expect_err);
}
