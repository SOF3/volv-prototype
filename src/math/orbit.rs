use super::*;

pub struct Orbit {}

impl Orbit {
    /// Calculate an orbit from the mass of the sun and the current ECI position+velocity of the
    /// planet
    pub fn from_mpv(mass: Mass, position: Vector, velocity: Vector) -> Self {
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

    pub fn time_reaching(&self, height: Length) -> Option<Time> {
        unimplemented!()
    }
}
