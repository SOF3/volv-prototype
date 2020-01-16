use std::borrow::Borrow;
use std::cmp;

use getset::*;

use super::body::*;
use crate::math::Time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventId(pub u32);

#[derive(Debug)]
pub struct Event {
    key: (EventId, Time),
    ty: EventType,
}

impl Event {
    pub fn id(&self) -> EventId {
        self.key.0
    }

    pub fn time(&self) -> Time {
        self.key.1
    }

    pub(super) fn into_type(self) -> EventType {
        self.ty
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.time()
            .cmp(&other.time())
            .then_with(|| self.id().cmp(&other.id()))
    }
}

impl Borrow<(EventId, Time)> for Event {
    fn borrow(&self) -> &(EventId, Time) {
        &self.key
    }
}

#[derive(Debug)]
pub(super) enum EventType {
    Collision(Collision),
    FieldChange(FieldChange),
}

#[derive(Debug, CopyGetters)]
pub(super) struct Collision {
    #[get_copy = "pub(super)"]
    pub(super) body1: BodyId,
    #[get_copy = "pub(super)"]
    pub(super) body2: BodyId,
}

#[derive(Debug, CopyGetters)]
pub(super) struct FieldChange {
    #[get_copy = "pub(super)"]
    pub(super) body: BodyId,
    #[get_copy = "pub(super)"]
    pub(super) from: LargeBodyId,
    #[get_copy = "pub(super)"]
    pub(super) to: LargeBodyId,
}
