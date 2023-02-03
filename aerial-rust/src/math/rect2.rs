use crate::math::line2::Line2;
use crate::math::vec2::Vec2;

pub struct RectLines {
    pub top: Line2,
    pub bottom: Line2,
    pub left: Line2,
    pub right: Line2,
}

#[derive(Default, Debug, Clone)]
pub struct Rect2 {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect2 {
    pub fn contains_point(&self, point: &Vec2) -> bool {
        if self.x < point.x && point.x < (self.x + self.w) {
            if self.y < point.y && point.y < (self.y + self.h) {
                return true;
            }
        }
        return false;
    }

    pub fn contains_rect(&self, rect: &Rect2) -> bool {
        if self.left() < rect.right() && self.right() > rect.left() {
            if self.top() < rect.bottom() && self.bottom() > rect.top() {
                return true;
            }
        }
        return false;
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.w
    }

    pub fn to_lines(&self) -> RectLines {
        RectLines {
            top: Line2 {
                // left top to right top
                point_a: Vec2 {
                    x: self.left(),
                    y: self.top(),
                },
                point_b: Vec2 {
                    x: self.right(),
                    y: self.top(),
                },
            },
            bottom: Line2 {
                // left bottom to right bottom
                point_a: Vec2 {
                    x: self.left(),
                    y: self.bottom(),
                },
                point_b: Vec2 {
                    x: self.right(),
                    y: self.bottom(),
                },
            },
            left: Line2 {
                // top left to bottom left
                point_a: Vec2 {
                    x: self.left(),
                    y: self.top(),
                },
                point_b: Vec2 {
                    x: self.left(),
                    y: self.bottom(),
                },
            },
            right: Line2 {
                // top right to bottom right
                point_a: Vec2 {
                    x: self.right(),
                    y: self.top(),
                },
                point_b: Vec2 {
                    x: self.right(),
                    y: self.bottom(),
                },
            },
        }
    }
}
