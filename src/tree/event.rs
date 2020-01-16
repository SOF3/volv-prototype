use std::borrow::Borrow;
use std::cmp;
use std::fmt;

use getset::*;

use super::*;
use crate::math::Time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventId(pub u32);

#[derive(Debug)]
pub struct Event<H: Handler> {
    key: (EventId, Time),
    ty: EventType<H>,
}

impl<H: Handler> Event<H> {
    pub fn id(&self) -> EventId {
        self.key.0
    }

    pub fn time(&self) -> Time {
        self.key.1
    }

    pub(super) fn into_type(self) -> EventType<H> {
        self.ty
    }
}

impl<H: Handler> PartialEq for Event<H> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<H: Handler> Eq for Event<H> {}

impl<H: Handler> PartialOrd for Event<H> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<H: Handler> Ord for Event<H> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.time()
            .cmp(&other.time())
            .then_with(|| self.id().cmp(&other.id()))
    }
}

impl<H: Handler> Borrow<(EventId, Time)> for Event<H> {
    fn borrow(&self) -> &(EventId, Time) {
        &self.key
    }
}

pub(super) enum EventType<H: Handler> {
    Collision(Collision),
    FieldChange(FieldChange),
    Misc(Box<dyn FnOnce(&mut System<H>)>),
}

impl<H: Handler> fmt::Debug for EventType<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Collision(c) => write!(f, "Collision({:?})", c),
            Self::FieldChange(fc) => write!(f, "FieldChange({:?})", fc),
            Self::Misc(_) => write!(f, "Misc(Fn)"),
        }
    }
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
