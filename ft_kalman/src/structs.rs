use nalgebra::Matrix3x1;
use std::fmt;

#[allow(non_snake_case)]
pub struct Axis {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

pub struct EulerAngles {
    pub φ: f64,
    pub θ: f64,
    pub ψ: f64
}

pub struct Motion {
    pub position: Vec<Axis>,
    pub speed: Vec<f64>,
    pub acceleration: Vec<Axis>,
    pub direction: Vec<EulerAngles>,
}

impl Axis {
    pub fn to_vec(&self) -> Matrix3x1<f64> {
        return Matrix3x1::new(self.X, self.Y, self.Z);
    }

    pub fn is_init(&self) -> bool {
        if self.X == 0.0 && self.Y == 0.0 && self.Z == 0.0 { return false }
        return true;
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.X, self.Y, self.Z)
    }
}

impl fmt::Display for EulerAngles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "φ: {} θ:{} ψ:{}", self.φ, self.θ, self.ψ)
    }
}