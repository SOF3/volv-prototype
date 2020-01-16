use super::*;
use crate::math::{Length, Mass, Vector};

pub trait Handler: Sized {
    fn on_collision(
        &mut self,
        body1: BodyRef<'_>,
        body2: BodyRef<'_>,
    ) -> (CollisionResolution, CollisionResolution);

    fn on_enter_subfield(&mut self, body: BodyRef<'_>, from: &LargeBody, to: &LargeBody);
    fn on_exit_subfield(&mut self, body: BodyRef<'_>, from: &LargeBody, to: &LargeBody);
}

pub enum CollisionResolution {
    Remove,
    Mutate(BodyMutation),
}

#[derive(Debug, Default)]
pub struct BodyMutation {
    pub surface_radius: Option<Length>,
    pub grav_radius: Option<Length>,
    pub mass: Option<Mass>,
    pub velocity: Option<Vector>,
}
