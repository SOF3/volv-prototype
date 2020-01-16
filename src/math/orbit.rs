use getset::*;

use super::*;

#[derive(Debug)]
pub struct Orbit {}

impl Orbit {
    /// Calculate an orbit from the mass of the sun and the current ECI position+velocity of the
    /// planet
    pub fn from_mpv(mass: Mass, eci: Eci) -> Self {
        unimplemented!()
    }

    pub fn eccentricity(&self) -> f64 {
        unimplemented!()
    }
    pub fn peripapsis(&self) -> Vector {
        unimplemented!()
    }
    pub fn apoapsis(&self) -> Vector {
        unimplemented!()
    }
    pub fn anomaly(&self) -> f64 {
        unimplemented!()
    }

    pub fn position(&self, t: Time) -> Vector {
        unimplemented!()
    }
    pub fn velocity(&self, t: Time) -> Vector {
        unimplemented!()
    }
    pub fn eci(&self, t: Time) -> Eci {
        Eci::new(self.position(t), self.velocity(t))
    }

    pub fn time_reaching(&self, height: Length) -> Option<Time> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, CopyGetters, serde::Serialize, serde::Deserialize)]
pub struct Eci {
    #[get_copy = "pub"]
    position: Vector,
    #[get_copy = "pub"]
    velocity: Vector,
}

impl Eci {
    pub fn new(position: Vector, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}
