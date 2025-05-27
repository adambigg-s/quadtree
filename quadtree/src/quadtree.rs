use glam::Vec2;

use crate::state::Particle;

#[derive(Debug)]
pub struct Rectangle {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rectangle {
    pub fn build(min: Vec2, max: Vec2) -> Self {
        Rectangle { min, max }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.min.x < point.x && self.min.y < point.y && self.max.x > point.x && self.max.y > point.y
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.max.x > other.min.x
            && self.min.x < other.max.x
            && self.max.y > other.min.y
            && self.min.y < other.max.y
    }
}

#[derive(Debug)]
pub struct TreeNode {
    pub children: Option<[usize; 4]>,
    pub bounds: Rectangle,
    pub data: Vec<usize>,
}

impl TreeNode {
    pub fn build(bounds: Rectangle) -> Self {
        TreeNode { children: None, bounds, data: Vec::new() }
    }
}

#[derive(Debug)]
pub struct QuadTreeWrap<'d, T> {
    // pub data: Option<&'d Vec<T>>,
    pub data: Option<&'d [T]>,
    pub nodes: Vec<TreeNode>,
    pub capacity: usize,
    pub head_node: TreeNode,
}

impl<'d> QuadTreeWrap<'d, Particle> {
    pub fn build(capacity: usize, bounds: Rectangle) -> QuadTreeWrap<'d, Particle> {
        QuadTreeWrap {
            data: None,
            nodes: Vec::new(),
            capacity,
            head_node: TreeNode { children: None, bounds, data: Vec::new() },
        }
    }

    pub fn init_tree(&mut self, data: &'d [Particle]) {
        self.data = Some(data);
    }
}
