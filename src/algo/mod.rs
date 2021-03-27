use std::rc::Rc;

use crate::graph_impl::DiGraph;
use crate::GraphError;
use petgraph::algo;
use petgraph::graph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SccGroups {
    #[wasm_bindgen(skip)]
    pub inner: Rc<Vec<Vec<graph::NodeIndex>>>,
}

#[wasm_bindgen]
pub struct SccGroup {
    #[wasm_bindgen(skip)]
    pub inner: Rc<Vec<Vec<graph::NodeIndex>>>,
    #[wasm_bindgen(readonly)]
    pub index: usize
}

#[wasm_bindgen]
impl SccGroups {
    fn new(val: Vec<Vec<graph::NodeIndex>>) -> Self {
        Self { inner: Rc::new(val) }
    }

    pub fn get(&self, index: usize) -> JsValue {
        match self.inner.len() > index {
            true => JsValue::from(SccGroup::new(self.inner.clone(), index)),
            false => JsValue::undefined(),
        }
    }
}

impl SccGroups {
    pub fn to_std_vec(&self) -> Vec<Vec<usize>> {
        self.inner.iter().map(|group| group.iter().map(|item| item.index()).collect()).collect()
    }
}


#[wasm_bindgen]
impl SccGroup {
    fn new(inner: Rc<Vec<Vec<graph::NodeIndex>>>, index: usize) -> Self {
        Self { inner: inner, index }
    }

    pub fn get(&self, index: usize) -> JsValue {
        match self.inner.get(self.index) {
            None => panic!("INTERNAL ERROR IN SCCGROUP"),
            Some(scc_group) => match scc_group.get(index) {
                Some(node_index) => JsValue::from(node_index.index() as u32),
                None => JsValue::undefined(),
            }
        }
    }

    pub fn get_all(&self, index: usize) -> JsValue {
        match self.inner.get(self.index) {
            None => panic!("INTERNAL ERROR IN SCCGROUP"),
            Some(scc_group) => match scc_group.get(index) {
                Some(node_index) => JsValue::from(node_index.index() as u32),
                None => JsValue::undefined(),
            }
        }
    }
}

//fn helper2(nodeIndexVec: Vec<graph::NodeIndex>) -> Vec<u32> {
//    nodeIndexVec.iter().map(|item| item.index() as u32).collect()
//}

#[wasm_bindgen(js_name = kosarajuScc)]
pub fn kosaraju_scc(graph: &DiGraph) -> SccGroups {
    let scc_groups: Vec<Vec<graph::NodeIndex>> = algo::kosaraju_scc(&graph.graph);
    return SccGroups::new(scc_groups)
}

#[wasm_bindgen(js_name = tarjan_scc)]
pub fn tarjan_scc(graph: &DiGraph) -> SccGroups {
    let scc_groups: Vec<Vec<graph::NodeIndex>> = algo::tarjan_scc(&graph.graph);
    return SccGroups::new(scc_groups)
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
