use crate::math::vec2::Vec2;

pub struct Line2 {
    pub point_a: Vec2,
    pub point_b: Vec2,
}

impl Line2 {
    pub fn intersects_line(&self, rhs: &Line2) -> Option<Vec2> {
        // http://www.realtimerendering.com/resources/GraphicsGems/gemsii/xlines.c

        let x1 = self.point_a.x;
        let y1 = self.point_a.y;
        let x2 = self.point_b.x;
        let y2 = self.point_b.y;

        let x3 = rhs.point_a.x;
        let y3 = rhs.point_a.y;
        let x4 = rhs.point_b.x;
        let y4 = rhs.point_b.y;

        /* Coefficients of line eqns. */

        /* Compute a1, b1, c1, where line joining points 1 and 2
         * is "a1 x  +  b1 y  +  c1  =  0".
         */

        let a1 = y2 - y1;
        let b1 = x1 - x2;
        let c1 = x2 * y1 - x1 * y2;

        /* 'Sign' values */

        /* Compute r3 and r4.
         */

        let r3 = a1 * x3 + b1 * y3 + c1;
        let r4 = a1 * x4 + b1 * y4 + c1;

        /* Check signs of r3 and r4.  If both point 3 and point 4 lie on
         * same side of line 1, the line segments do not intersect.
         */

        if r3 != 0.0 && r4 != 0.0 && has_same_signs(r3, r4) {
            // does NOT intersect
            return None;
        }

        /* Compute a2, b2, c2 */

        let a2 = y4 - y3;
        let b2 = x3 - x4;
        let c2 = x4 * y3 - x3 * y4;

        /* Compute r1 and r2 */

        let r1 = a2 * x1 + b2 * y1 + c2;
        let r2 = a2 * x2 + b2 * y2 + c2;

        /* Check signs of r1 and r2.  If both point 1 and point 2 lie
         * on same side of second line segment, the line segments do
         * not intersect.
         */

        if r1 != 0.0 && r2 != 0.0 && has_same_signs(r1, r2) {
            // does NOT intersect
            return None;
        }

        /* Line segments intersect: compute intersection point.
         */

        let denom = a1 * b2 - a2 * b1;
        if denom == 0.0 {
            // collinear
            return None;
        }

        let offset = {
            if denom < 0.0 {
                -denom / 2.0
            } else {
                denom / 2.0
            }
        };

        /* The denom/2 is to get rounding instead of truncating.  It
         * is added or subtracted to the numerator, depending upon the
         * sign of the numerator.
         */

        let num_x = b1 * c2 - b2 * c1;
        let result_x = {
            if num_x < 0.0 {
                num_x - offset
            } else {
                num_x + offset
            }
        } / denom;

        let num_y = a2 * c1 - a1 * c2;
        let result_y = {
            if num_y < 0.0 {
                num_y - offset
            } else {
                num_y + offset
            }
        } / denom;

        Some(Vec2 {
            x: result_x,
            y: result_y,
        })
    }
}

fn has_same_signs(a: f64, b: f64) -> bool {
    if a > 0.0 && b > 0.0 {
        return true;
    }
    if a < 0.0 && b < 0.0 {
        return true;
    }
    false
}
