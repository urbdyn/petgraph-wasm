use crate::utils;
use petgraph::graph;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Direction {
    Incoming = "incoming",
    Outgoing = "outgoing",
}

impl TryInto<petgraph::Direction> for Direction {
    type Error = JsValue;
    fn try_into(self) -> Result<petgraph::Direction, Self::Error> {
        match self {
            Direction::Incoming => Ok(petgraph::Direction::Incoming),
            Direction::Outgoing => Ok(petgraph::Direction::Outgoing),
            _ => Err(JsValue::from_str("Invalid direction")),
        }
    }
}

#[wasm_bindgen]
pub struct DiGraph {
    #[wasm_bindgen(skip)]
    pub graph: graph::DiGraph<JsValue, JsValue>,
}

#[wasm_bindgen]
impl DiGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(node_capacity: Option<usize>, edge_capacity: Option<usize>) -> Self {
        utils::set_panic_hook();
        DiGraph {
            graph: graph::DiGraph::with_capacity(
                node_capacity.or(Some(0)).unwrap(),
                edge_capacity.or(Some(0)).unwrap(),
            ),
        }
    }

    #[wasm_bindgen(js_name = nodeCount)]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    #[wasm_bindgen(js_name = edgeCount)]
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    #[wasm_bindgen(js_name = isDirected)]
    pub fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }

    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, weight: JsValue) -> usize {
        self.graph.add_node(weight).index()
    }

    #[wasm_bindgen(js_name = nodeWeight)]
    pub fn node_weight(&self, index: usize) -> Result<JsValue, JsValue> {
        match self.graph.node_weight(graph::NodeIndex::new(index)) {
            Some(weight) => Ok(weight.clone()),
            None => Err(JsValue::from_str("No node exists for given index")),
        }
    }

    #[wasm_bindgen(js_name = addEdge)]
    pub fn add_edge(&mut self, a: usize, b: usize, weight: JsValue) -> usize {
        self.graph
            .add_edge(graph::NodeIndex::new(a), graph::NodeIndex::new(b), weight)
            .index()
    }

    #[wasm_bindgen(js_name = updateEdge)]
    pub fn update_edge(&mut self, a: usize, b: usize, weight: JsValue) -> usize {
        self.graph
            .update_edge(graph::NodeIndex::new(a), graph::NodeIndex::new(b), weight)
            .index()
    }

    #[wasm_bindgen(js_name = edgeWeight)]
    pub fn edge_weight(&self, index: usize) -> Result<JsValue, JsValue> {
        match self.graph.edge_weight(graph::EdgeIndex::new(index)) {
            Some(weight) => Ok(weight.clone()),
            None => Err(JsValue::from_str("No edge exists for given index")),
        }
    }

    #[wasm_bindgen(js_name = edgeEndpoints)]
    pub fn edge_endpoints(&self, index: usize) -> Option<Vec<usize>> {
        self.graph
            .edge_endpoints(graph::EdgeIndex::new(index))
            .map(|(i1, i2)| vec![i1.index(), i2.index()])
    }

    #[wasm_bindgen(js_name = removeNode)]
    pub fn remove_node(&mut self, index: usize) -> Result<JsValue, JsValue> {
        match self.graph.remove_node(graph::NodeIndex::new(index)) {
            Some(node) => Ok(node),
            None => Err(JsValue::from_str("No node exists for given index")),
        }
    }

    #[wasm_bindgen(js_name = removeEdge)]
    pub fn remove_edge(&mut self, index: usize) -> Result<JsValue, JsValue> {
        match self.graph.remove_edge(graph::EdgeIndex::new(index)) {
            Some(edge) => Ok(edge),
            None => Err(JsValue::from_str("No edge exists for given index")),
        }
    }

    pub fn neighbors(&mut self, index: usize) -> Option<Vec<usize>> {
        let neighbor_vec: Vec<usize> = self
            .graph
            .neighbors(graph::NodeIndex::new(index))
            .map(|neighbor: graph::NodeIndex| neighbor.index())
            .collect();
        Some(neighbor_vec)
    }

    #[wasm_bindgen(js_name = neighborsDirected)]
    pub fn neighbors_directed(
        &mut self,
        index: usize,
        direction: Direction,
    ) -> Result<Vec<usize>, JsValue> {
        let node_index = graph::NodeIndex::new(index);
        let neighbor_vec: Vec<usize> = self
            .graph
            .neighbors_directed(node_index, direction.try_into()?)
            .map(|neighbor: graph::NodeIndex| neighbor.index())
            .collect();
        Ok(neighbor_vec)
    }

    #[wasm_bindgen(js_name = neighborsUndirected)]
    pub fn neighbors_undirected(&mut self, index: usize) -> Option<Vec<usize>> {
        let neighbor_vec: Vec<usize> = self
            .graph
            .neighbors_undirected(graph::NodeIndex::new(index))
            .map(|neighbor: graph::NodeIndex| neighbor.index())
            .collect();
        Some(neighbor_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_helpers::test::new_test_graph;
    use wasm_bindgen_test::*;

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
}
