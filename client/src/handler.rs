use volv::tree::{BodyRef, CollisionResolution, LargeBody};

pub struct Handler;

impl volv::tree::Handler for Handler {
    fn on_collision(
        &mut self,
        body1: BodyRef<'_>,
        body2: BodyRef<'_>,
    ) -> (CollisionResolution, CollisionResolution) {
        unimplemented!()
    }

    fn on_enter_subfield(&mut self, body: BodyRef<'_>, from: &LargeBody, to: &LargeBody) {
        unimplemented!()
    }
    fn on_exit_subfield(&mut self, body: BodyRef<'_>, from: &LargeBody, to: &LargeBody) {
        unimplemented!()
    }
}
