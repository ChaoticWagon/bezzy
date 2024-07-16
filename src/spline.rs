use crate::point::Point;
use crate::curve::CubicBezier;

pub struct BezierSpline {
    pub curves: Vec<CubicBezier>,
}

impl BezierSpline {
    pub fn get_point(&self, t: f64) -> Result<Point, &'static str> {
        if t < 0.0 || t > 1.0 {
            return Err("t must be between 0 and 1");
        }

        let mut t = t * self.curves.len() as f64;
        let curve_index = t as usize;
        t = t - curve_index as f64;

        self.curves[curve_index].get_point(t)
    }
}
// now I have to assert that the first curve's last point is the same as the second curve's first point
// and I also need a way to create a new curve from one curve and adding 3 points to it