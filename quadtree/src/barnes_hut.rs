use crate::quadtree::QuadTree;

pub struct BarnesHutWrapper<'d> {
    pub inner_tree: Option<&'d QuadTree>,
}

impl<'d> BarnesHutWrapper<'d> {
    pub fn new() -> BarnesHutWrapper<'d> {
        BarnesHutWrapper { inner_tree: None }
    }
}
