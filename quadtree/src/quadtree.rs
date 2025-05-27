use glam::Vec2;

use crate::state::Particle;

pub struct Rectangle {
    min: Vec2,
    max: Vec2,
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

pub struct TreeNode {
    children: Option<[usize; 4]>,
    bounds: Rectangle,
    data: Vec<usize>,
}

impl TreeNode {}

pub struct QuadTreeWrap<'d, T> {
    data: &'d Vec<T>,
    nodes: Vec<TreeNode>,
    capacity: usize,
    head_node: TreeNode,
}

impl<'d> QuadTreeWrap<'d, Particle> {
    pub fn build(data: &Vec<Particle>, capacity: usize, bounds: Rectangle) -> QuadTreeWrap<Particle> {
        QuadTreeWrap {
            data: data,
            nodes: Vec::new(),
            capacity: capacity,
            head_node: TreeNode { children: None, bounds: bounds, data: Vec::new() },
        }
    }

    pub fn init_tree(&mut self) {
        for particle in self.data {}
    }
}
