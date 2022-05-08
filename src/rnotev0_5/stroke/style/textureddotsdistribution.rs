use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
/// The distribution for the spread of dots across the width of a textured shape
pub enum TexturedDotsDistribution {
    /// Uniform distribution
    Uniform,
    /// Normal distribution
    Normal,
    /// Exponential distribution distribution, from the outline increasing in probability symmetrically to the center
    Exponential,
    /// Exponential distribution distribution, from the center increasing in probability symmetrically outwards to the outline
    ReverseExponential,
}

impl Default for TexturedDotsDistribution {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<crate::rnotev0_4::strokes::TexturedDotsDistribution> for TexturedDotsDistribution {
    fn from(dist: crate::rnotev0_4::strokes::TexturedDotsDistribution) -> Self {
        use crate::rnotev0_4::strokes::TexturedDotsDistribution as Tddv4;
        match dist {
            Tddv4::Uniform => Self::Uniform,
            Tddv4::Normal => Self::Normal,
            Tddv4::Exponential => Self::Exponential,
            Tddv4::ReverseExponential => Self::ReverseExponential,
        }
    }
}
