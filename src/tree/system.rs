use std::collections::{BTreeSet, HashMap};
use std::f64;

use super::*;
use crate::math::{Length, Time};

#[derive(Debug)]
pub struct System<H: Handler> {
    next_event_id: u64,
    next_body_id: u64,

    tree: Tree,
    event_queue: BTreeSet<Event>,

    handler: H,
}

fn next_id(id: &mut u64) -> u64 {
    let ret = *id;
    *id += 1;
    ret
}

impl<H: Handler> System<H> {
    pub fn from_schema(schema: LargeBodySchema, handler: H) -> Self {
        fn to_body(
            body_count: &mut u64,
            parent_index: &mut HashMap<BodyId, LargeBodyId>,
            schema: &LargeBodySchema,
        ) -> LargeBody {
            let id = LargeBodyId(BodyId(next_id(body_count)));
            let children = schema
                .children()
                .iter()
                .map(|schema| {
                    let body = to_body(body_count, parent_index, schema);
                    (body.id(), body)
                })
                .collect::<HashMap<_, _>>();
            for &child_id in children.keys() {
                parent_index.insert(child_id.0, id);
            }
            LargeBody {
                id,
                large: children,
                small: HashMap::new(),
                surface_radius: schema.surface_radius(),
                grav_radius: schema.grav_radius(),
                mass: schema.mass(),
            }
        }

        let mut body_count = 0u64;
        let mut parent_index = HashMap::new();
        let mut root = to_body(&mut body_count, &mut parent_index, &schema);

        root.grav_radius = Length(f64::INFINITY);

        let tree = Tree::new(root, parent_index);

        System {
            next_event_id: 0,
            next_body_id: body_count,
            tree,
            event_queue: BTreeSet::new(),
            handler,
        }
    }

    pub(super) fn next_event_id(&mut self) -> EventId {
        EventId(next_id(&mut self.next_event_id))
    }

    pub(super) fn next_body_id(&mut self) -> BodyId {
        BodyId(next_id(&mut self.next_body_id))
    }

    pub fn next_event(&self) -> Option<Time> {
        self.event_queue.iter().next().map(|event| event.time())
    }

    pub fn advance_event(&mut self, t: Time) {
        loop {
            let event = self.event_queue.iter().next();
            let event = match event {
                Some(event) => event,
                None => return,
            };
            if event.time() > t {
                return;
            }

            let key = (event.id(), event.time());
            let event = self
                .event_queue
                .take(&key)
                .expect("Event was obtained in quue");
            self.exec_event(event);
        }
    }

    fn exec_event(&mut self, event: Event) {
        match event.into_type() {
            EventType::Collision(event) => self.on_collision(event),
            EventType::FieldChange(event) => self.on_field_change(event),
        }
    }

    fn on_collision(&mut self, collision: Collision) {
        let body1 = self.tree.get_body(collision.body1());
        let body2 = self.tree.get_body(collision.body2());

        let (r1, r2) = self.handler.on_collision(body1, body2);
        unimplemented!()
    }

    fn on_field_change(&mut self, fc: FieldChange) {
        let body = self.tree.get_body(fc.body());
        let from = self.tree.get_large_body(fc.from());
        let to = self.tree.get_large_body(fc.to());

        unimplemented!()
    }
}

#[derive(Debug, derive_new::new)]
struct Tree {
    root: LargeBody,
    parent_index: HashMap<BodyId, LargeBodyId>,
}

impl Tree {
    pub fn get_large_body(&self, id: LargeBodyId) -> &LargeBody {
        if id == self.root.id() {
            return &self.root;
        }

        // We allow panic here, because BodyId should not be possible to create without a
        // corresponding object.
        // The ID may also be a dangling one, but then this implies we are dealing with a leak issue
        self.get_large_body(self.parent_index[id.as_ref()])
            .large
            .get(&id)
            .unwrap()
    }

    pub fn get_large_body_mut(&mut self, id: LargeBodyId) -> &mut LargeBody {
        if id == self.root.id() {
            return &mut self.root;
        }

        self.get_large_body_mut(self.parent_index[id.as_ref()])
            .large
            .get_mut(&id)
            .unwrap()
    }

    pub fn get_body<'t>(&'t self, id: BodyId) -> BodyRef<'t> {
        if id == self.root.id().0 {
            return BodyRef::Large(&self.root);
        }

        self.get_large_body(self.parent_index[&id])
            .get_child(id)
            .unwrap()
    }

    pub fn get_body_mut<'t>(&'t mut self, id: BodyId) -> BodyMut<'t> {
        if id == self.root.id().0 {
            return BodyMut::Large(&mut self.root);
        }

        self.get_large_body_mut(self.parent_index[&id])
            .get_child_mut(id)
            .unwrap()
    }
}
