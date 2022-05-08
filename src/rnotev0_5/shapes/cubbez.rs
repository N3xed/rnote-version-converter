use serde::{Deserialize, Serialize};

use super::line::Line;
use super::quadbez::QuadraticBezier;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "cubic_bezier")]
/// A cubic bezier curve
pub struct CubicBezier {
    #[serde(rename = "start")]
    /// the cubic curve start
    pub start: na::Vector2<f64>,
    #[serde(rename = "cp1")]
    /// the cubic curve first control point
    pub cp1: na::Vector2<f64>,
    #[serde(rename = "cp2")]
    /// the cubic curve second control point
    pub cp2: na::Vector2<f64>,
    #[serde(rename = "end")]
    /// the cubic curve end
    pub end: na::Vector2<f64>,
}

impl CubicBezier {
    /// tries to create a new cubic curve with the catmull-rom spline algorithm. Subsequent curves ( meaning, advancing the elements by one) have a smooth transition betwen them,
    /// and only being only affected by their four points (locality), so are easily calculated and easy to work with.
    /// making it a good spline to represent pen paths.
    /// See 'Conversion between Cubic Bezier Curves and Catmull-Rom Splines'
    pub fn new_w_catmull_rom(
        first: na::Vector2<f64>,
        second: na::Vector2<f64>,
        third: na::Vector2<f64>,
        forth: na::Vector2<f64>,
    ) -> Option<Self> {
        // Tension factor (tau)
        let tension = 1.0;

        // Creating cubic bezier with catmull-rom
        let start = second;
        let cp1 = second + (third - first) / (6.0 * tension);
        let cp2 = third - (forth - second) / (6.0 * tension);
        let end = third;

        let cubbez = CubicBezier {
            start,
            cp1,
            cp2,
            end,
        };

        // returns early when the cubbez does not have a length to prevent NaN when calculating the normals for segments with variable width
        if (cubbez.end - cubbez.start).magnitude() == 0.0 {
            return None;
        }

        Some(cubbez)
    }

    /// Split a cubic bezier into two at t where t > 0.0, < 1.0
    pub fn split(&self, t: f64) -> (CubicBezier, CubicBezier) {
        let a0 = self.start;
        let a1 = self.cp1;
        let a2 = self.cp2;
        let a3 = self.end;

        let b1 = a0.lerp(&a1, t);
        let a12 = a1.lerp(&a2, t);
        let b2 = b1.lerp(&a12, t);
        let c2 = a2.lerp(&a3, t);
        let c1 = a12.lerp(&c2, t);
        let b3 = b2.lerp(&c1, t);

        (
            CubicBezier {
                start: a0,
                cp1: b1,
                cp2: b2,
                end: b3,
            },
            CubicBezier {
                start: b3,
                cp1: c1,
                cp2: c2,
                end: a3,
            },
        )
    }

    /// Approximating a cubic bezier with a quadratic bezier
    pub fn approx_with_quadbez(&self) -> QuadraticBezier {
        let start = self.start;
        let cp = self.cp1.lerp(&self.cp2, 0.5);
        let end = self.end;

        QuadraticBezier { start, cp, end }
    }

    /// Approximating a cubic bezier with lines, given the number of splits
    pub fn approx_with_lines(&self, n_splits: i32) -> Vec<Line> {
        let mut lines = Vec::new();

        for i in 0..n_splits {
            let start_t = f64::from(i) / f64::from(n_splits);
            let end_t = f64::from(i + 1) / f64::from(n_splits);

            lines.push(Line {
                start: cubbez_calc(self.start, self.cp1, self.cp2, self.end, start_t),
                end: cubbez_calc(self.start, self.cp1, self.cp2, self.end, end_t),
            })
        }

        lines
    }
}

/// Calculates a point on a cubic curve given t ranging [0.0, 1.0]
pub fn cubbez_calc(
    p0: na::Vector2<f64>,
    p1: na::Vector2<f64>,
    p2: na::Vector2<f64>,
    p3: na::Vector2<f64>,
    t: f64,
) -> na::Vector2<f64> {
    let transform_matrix = na::matrix![
        1.0, 0.0, 0.0, 0.0;
        -3.0, 3.0, 0.0, 0.0;
        3.0, -6.0, 3.0, 0.0;
        -1.0, 3.0, -3.0, 1.0
    ];
    let p_matrix = na::matrix![
        p0[0], p0[1];
        p1[0], p1[1];
        p2[0], p2[1];
        p3[0], p3[1]
    ];

    (na::vector![1.0, t, t.powi(2), t.powi(3)].transpose() * transform_matrix * p_matrix)
        .transpose()
}
