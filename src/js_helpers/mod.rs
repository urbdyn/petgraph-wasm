//#[cfg(test)]
pub mod test;

use std::rc::Rc;

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
    pub index: usize,
}

impl SccGroups {
    pub fn new(val: Vec<Vec<graph::NodeIndex>>) -> Self {
        Self {
            inner: Rc::new(val),
        }
    }
}

#[wasm_bindgen]
impl SccGroups {
    pub fn get(&self, index: usize) -> JsValue {
        match self.inner.len() > index {
            true => JsValue::from(SccGroup::new(self.inner.clone(), index)),
            false => JsValue::undefined(),
        }
    }

    /// Create copy of SccGroups as `Array<Array<number>>`.
    /// This is a convenience method for workings with SccGroups!
    /// Using native Javascript `Array`s is much less memory efficient.
    /// Calling this function will produce a full copy using nested `Array`
    /// meaning this will greatly increase memory consumption.
    #[wasm_bindgen(js_name = toArrays)]
    pub fn to_arrays(&self) -> js_sys::Array {
        self.inner.iter().map(|scc_group| {
            //let tmp_vec = scc_group.iter().map(|x| x.index() as u32).collect::<Vec<u32>>();
            //js_sys::Uint32Array::from(&tmp_vec[..])
            let array = js_sys::Array::new_with_length(scc_group.len() as u32);
            scc_group.iter().enumerate().for_each(|(i, x)| {
                array.set(i as u32, JsValue::from(x.index() as u32));
            });
            return array;
        }).collect::<js_sys::Array>()
    }
}

impl SccGroups {
    pub fn copy_to_std_vec(&self) -> Vec<Vec<usize>> {
        self.inner
            .iter()
            .map(|group| group.iter().map(|item| item.index()).collect())
            .collect()
    }
}

#[wasm_bindgen]
impl SccGroup {
    fn new(inner: Rc<Vec<Vec<graph::NodeIndex>>>, index: usize) -> Self {
        Self {
            inner: inner,
            index,
        }
    }

    pub fn get(&self, index: usize) -> JsValue {
        match self.inner.get(self.index) {
            None => panic!("INTERNAL ERROR IN SCCGROUP"),
            Some(scc_group) => match scc_group.get(index) {
                Some(node_index) => JsValue::from(node_index.index() as u32),
                None => JsValue::undefined(),
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use graph::NodeIndex;
    use js_sys;
    use js_sys::Array;

    fn new_test_scc_groups() -> SccGroups {
        SccGroups::new(vec![
            vec![NodeIndex::new(0), NodeIndex::new(1), NodeIndex::new(2), NodeIndex::new(3)],
            vec![NodeIndex::new(4),],
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
        // JsValue of SccGroups
        let scc_groups = JsValue::from(new_test_scc_groups());
        assert!(scc_groups.is_object());
        // Retrieve the "get" function by name for scc_groups
        let scc_groups_get_raw: JsValue = js_sys::Reflect::get(&scc_groups, &js_str_get).unwrap();
        assert!(scc_groups_get_raw.is_function());
        let scc_groups_get = js_sys::Function::from(scc_groups_get_raw);

        // Call scc_groups.get(0) to get first scc_group object
        let scc_group0: JsValue = js_sys::Reflect::apply(&scc_groups_get, &scc_groups, &js_array_0).unwrap();
        assert!(scc_group0.is_object());
        // Retrieve the "get" function by name for scc_group
        let scc_group0_get_raw: JsValue = js_sys::Reflect::get(&scc_group0, &js_str_get).unwrap();
        assert!(scc_group0_get_raw.is_function());
        let scc_group0_get = js_sys::Function::from(scc_group0_get_raw);

        // Retrieve the first item from the 
        let scc_group0_item0: JsValue = js_sys::Reflect::apply(&scc_group0_get, &scc_group0, &js_array_0).unwrap();

        assert_eq!(scc_group0_item0, JsValue::from(0));
    }

    #[wasm_bindgen_test]
    fn can_access_scc_groups_to_arrays() {
        let scc_groups = new_test_scc_groups();
        // Create array representation of scc_groups
        let scc_groups_array: Array = scc_groups.to_arrays();
        assert!(Array::is_array(&scc_groups_array));

        // Get first group
        let scc_group0 = Array::from(&scc_groups_array.get(0));
        assert!(Array::is_array(&scc_group0));

        // Get second item of first group
        let scc_group0_item1 = scc_group0.get(1);
        assert_eq!(scc_group0_item1, JsValue::from(1));
    }

}
