use crate::graph_impl::DiGraph;
use crate::js_helpers::SccGroups;
use crate::GraphError;
use petgraph::algo;
use petgraph::graph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = kosarajuScc)]
pub fn kosaraju_scc(graph: &DiGraph) -> SccGroups {
    SccGroups::new(algo::kosaraju_scc(&graph.graph))
}

#[wasm_bindgen(js_name = tarjanScc)]
pub fn tarjan_scc(graph: &DiGraph) -> SccGroups {
    SccGroups::new(algo::tarjan_scc(&graph.graph))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_helpers::test::new_test_graph;
    use crate::{GraphError, GraphItemType};
    use wasm_bindgen_test::*;

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

    #[wasm_bindgen_test]
    fn can_get_kosaraju_scc() {
        let (mut g, _nodes, _edges) = new_test_graph();
        g.add_edge(2, 1, JsValue::NULL);
        let scc_groups: SccGroups = kosaraju_scc(&g);
        assert_eq!(
            scc_groups.copy_to_std_vec(),
            vec![vec![0, 3, 2, 1], vec![4]]
        );
    }

    #[wasm_bindgen_test]
    fn can_get_tarjan_scc() {
        let (mut g, _nodes, _edges) = new_test_graph();
        g.add_edge(2, 1, JsValue::NULL);
        let scc_groups: SccGroups = tarjan_scc(&g);
        assert_eq!(
            scc_groups.copy_to_std_vec(),
            vec![vec![3, 1, 2, 0], vec![4]]
        );
    }
}
