use crate::js_helpers::vec_tree::*;

use petgraph::graph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SccGroups {
    #[wasm_bindgen(skip)]
    pub vec_tree: VecTree<graph::NodeIndex, usize>,
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
    #[wasm_bindgen(js_name = getGroup)]
    pub fn get_group(&self, index: usize) -> Option<SccGroup> {
        match self.vec_tree.get(&[index]) {
            VecTreeElement::View(Some(view)) => Some(SccGroup { view }),
            VecTreeElement::View(None) => None,
            VecTreeElement::Item(_) => panic!("SccGroups::get_group returned item"),
        }
    }

    #[wasm_bindgen(js_name = length, getter)]
    pub fn len(&self) -> usize {
        match self.vec_tree.get(&[]) {
            VecTreeElement::View(Some(view)) => view.len(),
            VecTreeElement::View(None) => 0,
            VecTreeElement::Item(_) => panic!("SccGroups::len returned item"),
        }
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Create copy of SccGroups as `Array<Array<number>>`.
    /// This is a convenience method for workings with SccGroups!
    /// Using native Javascript `Array`s is less memory efficient.
    /// Calling this function will produce a full copy using nested `Array`
    /// meaning this will greatly increase memory consumption.
    #[wasm_bindgen(js_name = toArrays)]
    pub fn to_arrays(&self) -> js_sys::Array {
        match &*self.vec_tree.inner() {
            VecTreeInner::Vt2D(vt2d) => vt2d
                .data_iter()
                .map(|child_vec| {
                    child_vec
                        .iter()
                        .map(|x| JsValue::from(x.index() as u32))
                        .collect::<js_sys::Array>()
                })
                .collect::<js_sys::Array>(),
            _ => panic!("SccGroups has wrong dimensional VecTree"),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SccGroup {
    #[wasm_bindgen(skip)]
    pub view: VecTreeView<graph::NodeIndex, usize>,
}

#[wasm_bindgen]
impl SccGroup {
    #[wasm_bindgen(js_name = getItem)]
    pub fn get_item(&self, index: usize) -> Option<usize> {
        self.view.get_item(index)
    }

    #[wasm_bindgen(js_name = length, getter)]
    pub fn len(&self) -> usize {
        self.view.len()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.view.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::NodeIndex;
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
        // JS array => [0]
        let js_array_0 = js_sys::Array::of1(&JsValue::from(0u8));
        // JsValue of SccGroups1
        let scc_groups = JsValue::from(new_test_scc_groups());
        assert!(scc_groups.is_object());
        // Retrieve the "getGroup" function by name for scc_groups
        let scc_groups_get_group_raw: JsValue =
            js_sys::Reflect::get(&scc_groups, &JsValue::from_str("getGroup")).unwrap();
        assert!(scc_groups_get_group_raw.is_function());
        let scc_groups_get_group = js_sys::Function::from(scc_groups_get_group_raw);

        // Call scc_groups.getGroup(0) to get first scc_group object
        let scc_group0: JsValue =
            js_sys::Reflect::apply(&scc_groups_get_group, &scc_groups, &js_array_0).unwrap();
        assert!(scc_group0.is_object());
        // Retrieve the "getItem" function by name for scc_group
        let scc_group0_get_item_raw: JsValue =
            js_sys::Reflect::get(&scc_group0, &JsValue::from_str("getItem")).unwrap();
        assert!(scc_group0_get_item_raw.is_function());
        let scc_group0_get_item = js_sys::Function::from(scc_group0_get_item_raw);

        // Retrieve the first item from the first group
        let scc_group0_item0: JsValue =
            js_sys::Reflect::apply(&scc_group0_get_item, &scc_group0, &js_array_0).unwrap();

        assert_eq!(scc_group0_item0, JsValue::from(0));
    }
}
