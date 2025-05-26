use glam::Vec2;

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
}

struct QuadNode {
    pub id: usize,
    pub children: Option<[QuadNode; 4]>,
}

struct QuadTree<'a, T> {
    pub head: QuadNode,
    data: &'a Vec<T>,
}
