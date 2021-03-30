use crate::js_helpers::vec_tree::*;

use petgraph::graph;
use wasm_bindgen::prelude::*;

impl VecTreeItem for graph::NodeIndex {}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SccGroups {
    #[wasm_bindgen(skip)]
    pub vec_tree: VecTree<graph::NodeIndex, usize>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SccGroup {
    #[wasm_bindgen(skip)]
    pub view: VecTreeView<graph::NodeIndex, usize>,
}

impl SccGroups {
    /// Create SccGroups from 2D Vec of NodeIndex, as returned by SCC algorithms.
    pub fn new(data: Vec<Vec<graph::NodeIndex>>) -> Self {
        Self {
            vec_tree: VecTree::new2d(data, |x| x.index() as usize),
        }
    }

    /// Attempts to extract the `Vec`s from the inner `vec_tree`, which is inside an `Rc`.
    /// If this fails then self is returned.
    pub fn try_into_vecs(self) -> Result<Vec<Vec<graph::NodeIndex>>, Self> {
        match self.vec_tree.try_unwrap() {
            Ok(vti) => match vti {
                VecTreeInner::Vt2D(vt2d) => Ok(vt2d.into_vecs()),
                _ => panic!("SccGroups has wrong dimensional VecTree"),
            },
            Err(vec_tree) => Err(Self { vec_tree }),
        }
    }
}

#[wasm_bindgen]
impl SccGroups {
    pub fn get_group(&self, index: usize) -> Option<SccGroup> {
        match self.vec_tree.get(&vec![index]) {
            VecTreeElement::View(Some(view)) => Some(SccGroup { view }),
            VecTreeElement::View(None) => None,
            VecTreeElement::Item(_) => panic!("SccGroups::get_group returned item"),
        }
    }

    /// Create copy of SccGroups as `Array<Array<number>>`.
    /// This is a convenience method for workings with SccGroups!
    /// Using native Javascript `Array`s is less memory efficient.
    /// Calling this function will produce a full copy using nested `Array`
    /// meaning this will greatly increase memory consumption.
    #[wasm_bindgen(js_name = toArrays)]
    pub fn to_arrays(&self) -> js_sys::Array {
        match &*self.vec_tree.inner() {
            VecTreeInner::Vt2D(vt2d) => {
                let x = vt2d
                    .data_iter()
                    .map(|child_vec| {
                        child_vec
                            .iter()
                            .map(|x| JsValue::from(x.index() as u32))
                            .collect::<js_sys::Array>()
                    })
                    .collect::<js_sys::Array>();
                return x;
            }
            _ => panic!("SccGroups has wrong dimensional VecTree"),
        }
    }
}

#[wasm_bindgen]
impl SccGroup {
    pub fn get_item(&self, index: usize) -> Option<usize> {
        self.view.get_item(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::NodeIndex;
    use js_sys::Array;
    use wasm_bindgen_test::*;

    fn new_test_scc_groups() -> SccGroups {
        SccGroups::new(vec![
            vec![
                NodeIndex::new(0),
                NodeIndex::new(1),
                NodeIndex::new(2),
                NodeIndex::new(3),
            ],
            vec![NodeIndex::new(4)],
        ])
    }

    #[wasm_bindgen_test]
    fn can_access_scc_groups() {
        // JS string => "get"
        let js_str_get = JsValue::from_str("get");
        // JS array => [0]
        let js_array_0 = {
            let x = js_sys::Array::new();
            x.push(&JsValue::from(0));
            x
        };
    }
}
