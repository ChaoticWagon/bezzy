use crate::point::Point;

pub struct CubicBezier {
    pub points: [Point; 4]
}


impl CubicBezier {
    pub fn new(points: [Point; 4]) -> CubicBezier {
        CubicBezier {
            points
        }
    }

    pub fn get_point(&self, t: f64) -> Result<Point, &'static str> {
        if t < 0.0 || t > 1.0 {
            return Err("t must be between 0 and 1");
        }
        // t = 1.0 - t;
        let t3 = t * t * t;
        let t2 = t * t;

        Ok(&self.points[0] +
            &(t * (-3.0 * &self.points[0] + 3.0 * &self.points[1])) +
            t2 * (3.0 * &self.points[0] - 6.0 * &self.points[1] + 3.0 * &self.points[2]) +
            t3 * (&(-1.0 * &self.points[0] + 3.0 * &self.points[1] - 3.0 * &self.points[2]) + &self.points[3]))
    }

    // assert that the first curve's last point is the same as the second curve's first point
    pub fn is_continuous(&self, curve: &CubicBezier) -> bool {
        self.points[3] == curve.points[0]
    }
}