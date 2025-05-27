use glam::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.
    }

    pub fn split_quadrants(&self) -> [Self; 4] {
        let center = self.center();
        /* follows the unit circle quadrant conventions, but the origin is
        in the top left so it is a little confusing */
        [
            Rectangle::build(Vec2::new(center.x, self.min.y), Vec2::new(self.max.x, center.y)), // i
            Rectangle::build(self.min, center),                                                 // ii
            Rectangle::build(Vec2::new(self.min.x, center.y), Vec2::new(center.x, self.max.y)), // ii
            Rectangle::build(center, self.max),                                                 // iv
        ]
    }
}

pub fn random_vec2() -> Vec2 {
    Vec2::new(fastrand::f32(), fastrand::f32())
}
