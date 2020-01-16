use std::collections::HashMap;

use getset::*;

use crate::math::{Length, Mass};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BodyId(pub u64);

macro_rules! sl_body {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name(pub(super) BodyId);

        impl AsRef<BodyId> for $name {
            fn as_ref(&self) -> &BodyId {
                &self.0
            }
        }
    };
}
sl_body!(LargeBodyId);
sl_body!(SmallBodyId);

#[derive(Debug)]
pub enum Body {
    Large(LargeBody),
    Small(SmallBody),
}

impl Body {
    pub fn unwrap_large(self) -> LargeBody {
        match self {
            Body::Large(body) => body,
            Body::Small(_) => panic!("Expected large body, got small body"),
        }
    }

    pub fn unwrap_small(self) -> SmallBody {
        match self {
            Body::Large(_) => panic!("Expected small body, got large body"),
            Body::Small(body) => body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BodyRef<'t> {
    Large(&'t LargeBody),
    Small(&'t SmallBody),
}

impl<'t> BodyRef<'t> {
    pub fn borrow_large(self) -> &'t LargeBody {
        match self {
            Self::Large(body) => body,
            Self::Small(_) => panic!("Expected large body, got small body"),
        }
    }

    pub fn borrow_small(self) -> &'t SmallBody {
        match self {
            Self::Large(_) => panic!("Expected small body, got large body"),
            Self::Small(body) => body,
        }
    }
}

#[derive(Debug)]
pub enum BodyMut<'t> {
    Large(&'t mut LargeBody),
    Small(&'t mut SmallBody),
}

impl<'t> BodyMut<'t> {
    pub fn borrow_large(self) -> &'t mut LargeBody {
        match self {
            Self::Large(body) => body,
            Self::Small(_) => panic!("Expected large body, got small body"),
        }
    }

    pub fn borrow_small(self) -> &'t mut SmallBody {
        match self {
            Self::Large(_) => panic!("Expected small body, got large body"),
            Self::Small(body) => body,
        }
    }
}

#[derive(Debug, Getters, CopyGetters)]
pub struct LargeBody {
    #[get_copy = "pub"]
    pub(super) id: LargeBodyId,
    #[get = "pub"]
    pub(super) large: HashMap<LargeBodyId, LargeBody>,
    #[get = "pub"]
    pub(super) small: HashMap<SmallBodyId, SmallBody>,
    #[get_copy = "pub"]
    pub(super) surface_radius: Length,
    #[get_copy = "pub"]
    pub(super) grav_radius: Length,
    #[get_copy = "pub"]
    pub(super) mass: Mass,
}

impl LargeBody {
    pub fn get_child<'t>(&'t self, id: BodyId) -> Option<BodyRef<'t>> {
        if let Some(body) = self.large.get(&LargeBodyId(id)) {
            Some(BodyRef::Large(body))
        } else if let Some(body) = self.small.get(&SmallBodyId(id)) {
            Some(BodyRef::Small(body))
        } else {
            None
        }
    }

    pub fn get_child_mut<'t>(&'t mut self, id: BodyId) -> Option<BodyMut<'t>> {
        if let Some(body) = self.large.get_mut(&LargeBodyId(id)) {
            Some(BodyMut::Large(body))
        } else if let Some(body) = self.small.get_mut(&SmallBodyId(id)) {
            Some(BodyMut::Small(body))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SmallBody {
    id: SmallBodyId,
    mass: Mass,
    radius: Length,
}

#[derive(serde::Serialize, serde::Deserialize, Getters, CopyGetters)]
pub struct LargeBodySchema {
    #[get_copy = "pub"]
    surface_radius: Length,
    #[get_copy = "pub"]
    grav_radius: Length,
    #[get_copy = "pub"]
    mass: Mass,
    #[serde(default)]
    #[get = "pub"]
    children: Vec<LargeBodySchema>,
}
