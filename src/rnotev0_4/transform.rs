use serde::{Deserialize, Serialize};

/// To be used as state in a stroke to help implement the StrokeBehaviour trait
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "transform")]
pub struct Transform {
    #[serde(rename = "transform")]
    pub transform: na::Affine2<f64>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            transform: na::Affine2::identity(),
        }
    }
}

impl Transform {
    pub fn new(transform: na::Affine2<f64>) -> Self {
        Self { transform }
    }

    pub fn new_w_isometry(isometry: na::Isometry2<f64>) -> Self {
        Self {
            transform: na::convert(isometry),
        }
    }

    pub fn transform_point(&self, point: na::Point2<f64>) -> na::Point2<f64> {
        self.transform * point
    }

    pub fn append_translation_mut(&mut self, offset: na::Vector2<f64>) {
        self.transform = na::Translation2::from(offset) * self.transform;
    }

    pub fn append_rotation_wrt_point_mut(&mut self, angle: f64, center: na::Point2<f64>) {
        self.transform = na::Translation2::from(-center.coords) * self.transform;
        self.transform = na::Rotation2::new(angle) * self.transform;
        self.transform = na::Translation2::from(center.coords) * self.transform;
    }

    pub fn append_scale_mut(&mut self, scale: na::Vector2<f64>) {
        let translation = (self.transform * na::point![0.0, 0.0]).coords;

        self.transform = na::Translation2::from(-translation) * self.transform;

        self.transform = na::try_convert(
            na::Scale2::<f64>::from(scale).to_homogeneous() * self.transform.to_homogeneous(),
        )
        .unwrap();

        self.transform = na::Translation2::from(translation) * self.transform;
    }

    pub fn to_svg_transform_attr_str(&self) -> String {
        let matrix = self.transform;

        format!(
            "matrix({:.3} {:.3} {:.3} {:.3} {:.3} {:.3})",
            matrix[(0, 0)],
            matrix[(1, 0)],
            matrix[(0, 1)],
            matrix[(1, 1)],
            matrix[(0, 2)],
            matrix[(1, 2)],
        )
    }
}
