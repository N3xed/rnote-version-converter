use serde::{Deserialize, Serialize};

use super::line::Line;
use super::CubicBezier;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "quadratic_bezier")]
/// A quadratic bezier curve
pub struct QuadraticBezier {
    #[serde(rename = "start")]
    /// The curve start
    pub start: na::Vector2<f64>,
    #[serde(rename = "cp")]
    /// The curve control point
    pub cp: na::Vector2<f64>,
    #[serde(rename = "end")]
    /// The curve end
    pub end: na::Vector2<f64>,
}

impl QuadraticBezier {
    /// Returns offset distance of a quadratic bezier at t where t > 0.0, < 1.0. Currently a linear interpolation between the start and end offset.
    /// TODO: finding a better algorithm based on curve length
    pub fn quadbez_calc_offset_dist_at_t(
        &self,
        start_offset_dist: f64,
        end_offset_dist: f64,
        t: f64,
    ) -> f64 {
        start_offset_dist + (end_offset_dist - start_offset_dist) * t
    }

    /// Split a quadratic bezier curve into two quadratics, at interpolation value z: between 0.0 and 1.0
    pub fn split(&self, z: f64) -> (QuadraticBezier, QuadraticBezier) {
        let p0 = self.start;
        let p1 = self.cp;
        let p2 = self.end;

        let first_splitted = QuadraticBezier {
            start: p0,
            cp: z * p1 - (z - 1.0) * p0,
            end: z.powi(2) * p2 - 2.0 * z * (z - 1.0) * p1 + (z - 1.0).powi(2) * p0,
        };

        let second_splitted = QuadraticBezier {
            start: z.powi(2) * p2 - 2.0 * z * (z - 1.0) * p1 + (z - 1.0).powi(2) * p0,
            cp: z * p2 - (z - 1.0) * p1,
            end: p2,
        };

        (first_splitted, second_splitted)
    }

    /// convert to a cubic bezier ( raising the order is without losses)
    pub fn to_cubic_bezier(&self) -> CubicBezier {
        CubicBezier {
            start: self.start,
            cp1: self.start + (2.0 / 3.0) * (self.cp - self.start),
            cp2: self.end + (2.0 / 3.0) * (self.cp - self.end),
            end: self.end,
        }
    }

    /// Approximating a quadratic bezier with lines, given the number of splits distributed evenly based on the t value.
    pub fn approx_with_lines(&self, n_splits: i32) -> Vec<Line> {
        let mut lines = Vec::new();

        for i in 0..n_splits {
            let start_t = f64::from(i) / f64::from(n_splits);
            let end_t = f64::from(i + 1) / f64::from(n_splits);

            lines.push(Line {
                start: quadbez_calc(self.start, self.cp, self.end, start_t),
                end: quadbez_calc(self.start, self.cp, self.end, end_t),
            })
        }

        lines
    }
}

/// Coefficient a of quadratic bezier in polynomial form: C = a * t^2 + b * t + c
pub fn quadbez_coeff_a(
    p0: na::Vector2<f64>,
    p1: na::Vector2<f64>,
    p2: na::Vector2<f64>,
) -> na::Vector2<f64> {
    p2 - 2.0 * p1 + p0
}

/// Coefficient b of quadratic bezier in polynomial form: C = a * t^2 + b * t + c
pub fn quadbez_coeff_b(p0: na::Vector2<f64>, p1: na::Vector2<f64>) -> na::Vector2<f64> {
    2.0 * p1 - 2.0 * p0
}

/// Coefficient c of quadratic bezier in polynomial form: C = a * t^2 + b * t + c
#[allow(dead_code)]
pub fn quadbez_coeff_c(p0: na::Vector2<f64>) -> na::Vector2<f64> {
    p0
}

/// calculating the value of a bezier curve with its support points, for t: between 0.0 and 1.0
#[allow(dead_code)]
pub fn quadbez_calc(
    p0: na::Vector2<f64>,
    p1: na::Vector2<f64>,
    p2: na::Vector2<f64>,
    t: f64,
) -> na::Vector2<f64> {
    quadbez_coeff_a(p0, p1, p2) * t.powi(2) + quadbez_coeff_b(p0, p1) * t + quadbez_coeff_c(p0)
}

/// Coefficient a of quadratic bezier derivation in polynomial form: C' = a * t + b
pub fn quad_bezier_derive_coeff_a(
    p0: na::Vector2<f64>,
    p1: na::Vector2<f64>,
    p2: na::Vector2<f64>,
) -> na::Vector2<f64> {
    2.0 * p2 - 4.0 * p1 + 2.0 * p0
}

/// Coefficient b of quadratic bezier derivation in polynomial form: C' = a * t + b
pub fn quadbez_derive_coeff_b(p0: na::Vector2<f64>, p1: na::Vector2<f64>) -> na::Vector2<f64> {
    2.0 * p1 - 2.0 * p0
}

#[allow(dead_code)]
/// calculating the derivative of the bezier curve for t: between 0.0 and 1.0
pub fn quadbez_derive_calc(
    p0: na::Vector2<f64>,
    p1: na::Vector2<f64>,
    p2: na::Vector2<f64>,
    t: f64,
) -> na::Vector2<f64> {
    quad_bezier_derive_coeff_a(p0, p1, p2) * t + quadbez_derive_coeff_b(p0, p1)
}
