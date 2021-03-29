//! `VecTree` and friends are a way to traverse arbitrary dimensional
//! (currently 1 to 3) nested `Vec`s. It is used to take away the boilerplate
//! from creating JS WASM friendly access structs over nested `Vec`s, which are
//! common in petgraph.

use std::fmt;
use std::rc::Rc;

/// Accessor over a vector tree of various depth/dimension (currently 1 to 3)
///
/// * Type `T1` is the Type of the data in the nested Vec.
/// * Type `T2` is the Type returned when accessing the data, via transform.
/// * You can have `T1` and `T2` be the same and the transform be an identity
///   function if no transform is needed.
///
/// Example usage:
/// ```rust
/// use petgraph_wasm::js_helpers::vec_tree::*;
/// // Create vec
/// let vec1d = vec![0,1,2,3];
/// // 1D VecTree that returns values as bool based on whether they're even.
/// let vt1d_to_bool = VecTree::new1d(vec1d, |x| x % 2 == 0);
/// // Get the second element
/// let vt1d_to_bool_item2 = vt1d_to_bool.get(&vec![1]).unwrap_as_item();
/// assert_eq!(vt1d_to_bool_item2, Some(false));
/// ```
#[derive(Debug)]
pub struct VecTree<T1: Clone, T2 = T1> {
    inner: Rc<VecTreeInner<T1, T2>>,
}

/// Manually implemented as T2 doesn't necessarily implement `Clone` and thus
/// it can't be derived.
impl<T1: Clone, T2> Clone for VecTree<T1, T2> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T1: Clone, T2> VecTree<T1, T2> {
    /// Create a VecTree from a 1d vec (`Vec<T1>`) with transform which is
    /// applied to values when retrieved.
    pub fn new1d(data: Vec<T1>, transform: fn(&T1) -> T2) -> Self {
        Self {
            inner: Rc::new(VecTreeInner::Vt1D(VecTree1 { data, transform })),
        }
    }

    /// Create a VecTree from a 2d vec (`Vec<Vec<T1>>`) with transform which is
    /// applied to values when retrieved.
    pub fn new2d(data: Vec<Vec<T1>>, transform: fn(&T1) -> T2) -> Self {
        Self {
            inner: Rc::new(VecTreeInner::Vt2D(VecTree2 { data, transform })),
        }
    }

    /// Create a VecTree from a 3d vec (`Vec<Vec<Vec<T1>>>`) with transform which is
    /// applied to values when retrieved.
    pub fn new3d(data: Vec<Vec<Vec<T1>>>, transform: fn(&T1) -> T2) -> Self {
        Self {
            inner: Rc::new(VecTreeInner::Vt3D(VecTree3 { data, transform })),
        }
    }

    /// Attempt to unwrap self as `VecTreeInner` from `Rc` or return `Self` on
    /// failure.
    pub fn try_unwrap(self) -> Result<VecTreeInner<T1, T2>, Self> {
        Rc::try_unwrap(self.inner).map_err(|inner| Self { inner })
    }

    /// Return `Rc` cloned ref of inner `VecTreeInner`. Useful for when needing to
    /// directly traverse the `Vec`s.
    pub fn inner(&self) -> Rc<VecTreeInner<T1, T2>> {
        self.inner.clone()
    }

    /// Get's depth/dimensions of `VecTree`
    pub fn depth(&self) -> usize {
        self.inner.depth()
    }

    /// Takes a "position" vector which is the index's to traverse from the top
    /// down through the `VecTree`. If the length of positions is more than the
    /// VecTree has then it will `panic!`. Else returns `VecTreeElement` which
    /// is either a `View` struct which can be used to further access its
    /// children or an `Item` which is the child of a leaf `Vec`. Both are
    /// `Option`s as the requested index may or may not exist.
    ///
    /// Note that calling `get` with position being an empty `Vec` will return
    /// a `VecTreeView` over the entire `VecTree`.
    pub fn get(&self, position: &[usize]) -> VecTreeElement<T1, T2> {
        match self.inner.as_ref() {
            VecTreeInner::Vt1D(vt1d) => match position.len() {
                0 => VecTreeElement::View(Some(VecTreeView::new(self, position))),
                1 => VecTreeElement::Item(vt1d.get_item(position)),
                _ => panic!("accessing 1d VecTree, position should have length 0 to 1"),
            },
            VecTreeInner::Vt2D(vt2d) => match position.len() {
                0 | 1 => VecTreeElement::View(Some(VecTreeView::new(self, position))),
                2 => VecTreeElement::Item(vt2d.get_item(position)),
                _ => panic!("accessing 2d VecTree, position should have length 0 to 2"),
            },
            VecTreeInner::Vt3D(vt3d) => match position.len() {
                0 | 1 | 2 => VecTreeElement::View(Some(VecTreeView::new(self, position))),
                3 => VecTreeElement::Item(vt3d.get_item(position)),
                _ => panic!("accessing 3d VecTree, position should have length 0 to 3"),
            },
        }
    }

    /// Gets the length of the `Vec` a "position" which is the indexes to traverse
    /// from the top down through the `VecTree`. If the length of positions is
    /// more than the VecTree has then it will `panic!`. Else returns `Option<usize>`
    /// which is `None` if the `position` doesn't exist in the `VecTree`.
    ///
    /// Note that calling `get_len` with position being an empty `Vec` will return
    /// length of the root node of the `VecTree`.
    pub fn get_len(&self, position: &Vec<usize>) -> Option<usize> {
        match self.inner.as_ref() {
            VecTreeInner::Vt1D(vt1d) => match position.len() {
                0 => Some(vt1d.data.len()),
                _ => panic!("to get len of 1d VecTree, position should have length 0"),
            },
            VecTreeInner::Vt2D(vt2d) => match position.len() {
                0 => Some(vt2d.data.len()),
                1 => vt2d.data.get(position[0]).and_then(|x| Some(x.len())),
                _ => panic!("to get len of 2d VecTree, position should have length 0 to 1"),
            },
            VecTreeInner::Vt3D(vt3d) => match position.len() {
                0 => Some(vt3d.data.len()),
                1 => vt3d.data.get(position[0]).and_then(|x| Some(x.len())),
                2 => vt3d
                    .data
                    .get(position[0])
                    .and_then(|x| x.get(position[1]))
                    .and_then(|x| Some(x.len())),
                _ => panic!("to get len of 3d VecTree, position should have length 0 to 2"),
            },
        }
    }
}

/// Inner storage for VecTree
pub enum VecTreeInner<T1: Clone, T2> {
    Vt1D(VecTree1<T1, T2>),
    Vt2D(VecTree2<T1, T2>),
    Vt3D(VecTree3<T1, T2>),
}

impl<T1: Clone, T2> VecTreeInner<T1, T2> {
    /// Returns the depth of the `VecTreeInner`
    pub fn depth(&self) -> usize {
        match self {
            VecTreeInner::Vt1D(_) => 1,
            VecTreeInner::Vt2D(_) => 2,
            VecTreeInner::Vt3D(_) => 3,
        }
    }
}

impl<T1: Clone, T2> fmt::Debug for VecTreeInner<T1, T2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (vti_type, len) = match self {
            VecTreeInner::Vt1D(vt1d) => ("Vt1D", vt1d.len()),
            VecTreeInner::Vt2D(vt2d) => ("Vt2D", vt2d.len()),
            VecTreeInner::Vt3D(vt3d) => ("Vt3D", vt3d.len()),
        };
        f.debug_struct("VecTreeInner")
            .field("type", &vti_type)
            .field("len", &len)
            .finish()
    }
}

/// Trait over VecTree storage objects for particular dimensionality.
pub trait VecTreeN<T> {
    type Child;

    /// Get the item for the position
    fn get_item(&self, position: &[usize]) -> Option<T>;
    /// Get length for top level of `VecTree`
    fn len(&self) -> usize;
    /// Consume the `VecTreeN` and return its inner vec structure.
    fn into_vecs(self) -> Vec<Self::Child>;
    /// Get iterator over internal vec data.
    fn data_iter(&self) -> std::slice::Iter<'_, Self::Child>;
}

/// VecTree for depth 1
pub struct VecTree1<T1: Clone, T2> {
    data: Vec<T1>,
    transform: fn(&T1) -> T2,
}
impl<T1: Clone, T2> VecTreeN<T2> for VecTree1<T1, T2> {
    type Child = T1;

    fn get_item(&self, position: &[usize]) -> Option<T2> {
        match position.len() {
            1 => self
                .data
                .get(position[0])
                .and_then(|x| Some((self.transform)(x))),
            _ => panic!("get_item for 1d VecTree, position should have length 1"),
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn into_vecs(self) -> Vec<Self::Child> {
        self.data
    }
    fn data_iter(&self) -> std::slice::Iter<'_, Self::Child> {
        self.data.iter()
    }
}

/// VecTree for depth 2
pub struct VecTree2<T1: Clone, T2> {
    data: Vec<Vec<T1>>,
    transform: fn(&T1) -> T2,
}
impl<T1: Clone, T2> VecTreeN<T2> for VecTree2<T1, T2> {
    type Child = Vec<T1>;

    fn get_item(&self, position: &[usize]) -> Option<T2> {
        match position.len() {
            2 => self
                .data
                .get(position[0])
                .and_then(|x| x.get(position[1]))
                .and_then(|x| Some((self.transform)(x))),
            _ => panic!("get_item for 2d VecTree, position should have length 2"),
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn into_vecs(self) -> Vec<Self::Child> {
        self.data
    }
    fn data_iter(&self) -> std::slice::Iter<'_, Self::Child> {
        self.data.iter()
    }
}

/// VecTree for depth 3
pub struct VecTree3<T1: Clone, T2> {
    data: Vec<Vec<Vec<T1>>>,
    transform: fn(&T1) -> T2,
}
impl<T1: Clone, T2> VecTreeN<T2> for VecTree3<T1, T2> {
    type Child = Vec<Vec<T1>>;

    fn get_item(&self, position: &[usize]) -> Option<T2> {
        match position.len() {
            3 => self
                .data
                .get(position[0])
                .and_then(|x| x.get(position[1]))
                .and_then(|x| x.get(position[2]))
                .and_then(|x| Some((self.transform)(x))),
            _ => panic!("get_item for 3d VecTree, position should have length 3"),
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn into_vecs(self) -> Vec<Self::Child> {
        self.data
    }
    fn data_iter(&self) -> std::slice::Iter<'_, Self::Child> {
        self.data.iter()
    }
}

/// Either a view of a `VecTree` of one of its `Clone`s
pub enum VecTreeElement<T1: Clone, T2> {
    View(Option<VecTreeView<T1, T2>>),
    Item(Option<T2>),
}
impl<T1: Clone, T2> VecTreeElement<T1, T2> {
    /// Attempt to unwrap to `VecTreeElement::View` or `panic!`
    pub fn unwrap_as_view(self) -> Option<VecTreeView<T1, T2>> {
        match self {
            VecTreeElement::View(v) => v,
            _ => panic!("Attempted VecTreeElement::unwrap_as_view on non-view element"),
        }
    }
    /// Attempt to unwrap to `VecTreeElement::Item` or `panic!`
    pub fn unwrap_as_item(self) -> Option<T2> {
        match self {
            VecTreeElement::Item(i) => i,
            _ => panic!("Attempted VecTreeElement::unwrap_as_item on non-item element"),
        }
    }
}

/// View from a particular location in a `VecTree`.
/// This is a light weight struct which allows for exploring and interacting
/// with a `VecTree`.
#[derive(Debug)]
pub struct VecTreeView<T1: Clone, T2 = T1> {
    vec_tree: VecTree<T1, T2>,
    position: Vec<usize>,
}

impl<T1: Clone, T2> VecTreeView<T1, T2> {
    fn new(vec_tree: &VecTree<T1, T2>, position: &[usize]) -> Self {
        Self {
            vec_tree: vec_tree.clone(),
            position: position.to_vec(),
        }
    }

    /// Gets the current depth `VecTreeView` relative to the `VecTree`.
    /// Zero indexed, so one layer "down" from the root node is depth 1.
    pub fn current_depth(&self) -> usize {
        self.position.len()
    }

    /// Gets the max depth of the `VecTree`.
    /// Zero indexed and only counts Vecs, so one `Vec` layer "down" from
    /// the root node is depth 1.
    pub fn max_depth(&self) -> usize {
        self.vec_tree.inner().depth() - 1
    }

    /// If the view is at the root node.
    pub fn is_root(&self) -> bool {
        self.position.is_empty()
    }

    /// If the view is a leaf-`Vec` in the `Vec` tree.
    pub fn is_leaf(&self) -> bool {
        self.vec_tree.inner().depth() - 1 == self.position.len()
    }

    /// Get len for `Vec` selected by View
    pub fn len(&self) -> usize {
        self.vec_tree
            .get_len(&self.position)
            .expect("Failed to get length for VecTreeView")
    }

    /// Get View at given position or panic if it's an Item
    pub fn get_view(&self, index: usize) -> Option<Self> {
        let mut position = self.position.clone();
        position.push(index);
        match self.vec_tree.get(&position) {
            VecTreeElement::Item(_) => panic!("got item instead of view"),
            VecTreeElement::View(v) => v,
        }
    }

    /// Get Item at given position or panic if it's a View
    pub fn get_item(&self, index: usize) -> Option<T2> {
        let mut position = self.position.clone();
        position.push(index);
        match self.vec_tree.get(&position) {
            VecTreeElement::View(_) => panic!("got view instead of item"),
            VecTreeElement::Item(i) => i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use wasm_bindgen_test::*;

    fn new_test_vec_tree_1() -> VecTree<usize> {
        //VecTree::new1d(vec![vec![0, 1, 2, 3], vec![4]])
        VecTree::new1d(vec![0, 1, 2, 3], |x| *x)
    }
    fn new_test_vec_tree_2() -> VecTree<usize> {
        VecTree::new2d(vec![vec![0, 1, 2, 3], vec![4]], |x| *x)
    }
    fn new_test_vec_tree_3() -> VecTree<usize> {
        VecTree::new3d(
            vec![
                vec![vec![0, 1, 2, 3], vec![4]],
                vec![vec![5, 6], vec![7, 8]],
            ],
            |x| *x,
        )
    }

    #[test]
    fn vtnd_can_get_depth() {
        assert_eq!(new_test_vec_tree_1().depth(), 1);
        assert_eq!(new_test_vec_tree_2().depth(), 2);
        assert_eq!(new_test_vec_tree_3().depth(), 3);
    }

    #[test]
    fn vt1d_can_get_len_for_positions() {
        let vt1d = new_test_vec_tree_1();
        assert_eq!(vt1d.get_len(&vec![]), Some(4));
    }

    #[test]
    fn vt2d_can_get_len_for_positions() {
        let vt2d = new_test_vec_tree_2();
        assert_eq!(vt2d.get_len(&vec![]), Some(2));
        assert_eq!(vt2d.get_len(&vec![0]), Some(4));
        assert_eq!(vt2d.get_len(&vec![2]), None);
    }

    #[test]
    fn vt3d_can_get_len_for_positions() {
        let vt3d = new_test_vec_tree_3();
        assert_eq!(vt3d.get_len(&vec![]), Some(2));
        assert_eq!(vt3d.get_len(&vec![0]), Some(2));
        assert_eq!(vt3d.get_len(&vec![2]), None);
        assert_eq!(vt3d.get_len(&vec![0, 0]), Some(4));
        assert_eq!(vt3d.get_len(&vec![0, 2]), None);
    }

    #[test]
    fn vt1d_can_get_view_for_positions() {
        let vt1d = new_test_vec_tree_1();

        let vt1d_view1 = vt1d.get(&vec![]).unwrap_as_view().unwrap();
        assert_eq!(vt1d_view1.is_root(), true);
        assert_eq!(vt1d_view1.is_leaf(), true);
        assert_eq!(vt1d_view1.len(), 4);
        assert_eq!(vt1d_view1.current_depth(), 0);
        assert_eq!(vt1d_view1.max_depth(), 0);

        let vt1d_view2 = vt1d.get(&vec![3]).unwrap_as_item();
        assert_eq!(vt1d_view2, Some(3));
    }

    #[test]
    fn vt2d_can_get_view_for_positions() {
        let vt2d = new_test_vec_tree_2();
        {
            let vt2d_view1 = vt2d.get(&vec![]).unwrap_as_view().unwrap();
            assert_eq!(vt2d_view1.is_root(), true);
            assert_eq!(vt2d_view1.is_leaf(), false);
            assert_eq!(vt2d_view1.len(), 2);
            assert_eq!(vt2d_view1.current_depth(), 0);
            assert_eq!(vt2d_view1.max_depth(), 1);
        }
        {
            let vt2d_view2 = vt2d.get(&vec![0]).unwrap_as_view().unwrap();
            assert_eq!(vt2d_view2.is_root(), false);
            assert_eq!(vt2d_view2.is_leaf(), true);
            assert_eq!(vt2d_view2.len(), 4);
            assert_eq!(vt2d_view2.current_depth(), 1);
            assert_eq!(vt2d_view2.max_depth(), 1);
        }
        {
            let vt2d_view3 = vt2d.get(&vec![0, 3]).unwrap_as_item();
            assert_eq!(vt2d_view3, Some(3));
        }
    }

    #[test]
    fn vt3d_can_get_view_for_positions() {
        let vt3d = new_test_vec_tree_3();
        {
            let vt3d_view1 = vt3d.get(&vec![]).unwrap_as_view().unwrap();
            assert_eq!(vt3d_view1.is_root(), true);
            assert_eq!(vt3d_view1.is_leaf(), false);
            assert_eq!(vt3d_view1.len(), 2);
            assert_eq!(vt3d_view1.current_depth(), 0);
            assert_eq!(vt3d_view1.max_depth(), 2);
        }
        {
            let vt3d_view2 = vt3d.get(&vec![0]).unwrap_as_view().unwrap();
            assert_eq!(vt3d_view2.is_root(), false);
            assert_eq!(vt3d_view2.is_leaf(), false);
            assert_eq!(vt3d_view2.len(), 2);
            assert_eq!(vt3d_view2.current_depth(), 1);
            assert_eq!(vt3d_view2.max_depth(), 2);
        }
        {
            let vt3d_view3 = vt3d.get(&vec![0, 1]).unwrap_as_view().unwrap();
            assert_eq!(vt3d_view3.is_root(), false);
            assert_eq!(vt3d_view3.is_leaf(), true);
            assert_eq!(vt3d_view3.len(), 1);
            assert_eq!(vt3d_view3.current_depth(), 2);
            assert_eq!(vt3d_view3.max_depth(), 2);
        }
        {
            let vt3d_view4 = vt3d.get(&vec![0, 1, 0]).unwrap_as_item();
            assert_eq!(vt3d_view4, Some(4));
        }
    }
}
