use volv::math::{Length, Time, Vector};
use volv::tree::{BodyId, Handler, LargeBody, LargeBodyId, SmallBody, System};

#[derive(Debug)]
pub struct Viewport {
    pub at: Vector,
    pub of: LargeBodyId,
    /// The lengths of the sides of the viewport
    pub dim: (Length, Length),
}

impl Viewport {
    fn convert_pos(&self, pos: Vector) -> ViewportCoord {
        let relative = pos - self.at;
        (
            0.5 + (relative[0] / self.dim.0 .0) as f32,
            0.5 + (relative[1] / self.dim.1 .0) as f32,
        )
    }
}

pub type ViewportCoord = (f32, f32);

pub trait Renderer: Sized {
    /// The scale is the viewport length (in [0, 1]) for an object of Length(1.0) along each axis
    fn render_body(&mut self, id: BodyId, at: ViewportCoord, scale: (f32, f32));

    fn proxy<'t>(
        &'t mut self,
        center: ViewportCoord,
        scale: f32,
    ) -> Option<ProxyRenderer<'t, Self>> {
        let ret = ProxyRenderer {
            inner: self,
            center,
            scale,
        };
        ret.oob_opt()
    }
}

#[derive(Debug)]
pub struct ProxyRenderer<'t, R: Renderer> {
    inner: &'t mut R,
    center: ViewportCoord,
    scale: f32,
}

impl<'t, R: Renderer> ProxyRenderer<'t, R> {
    fn translate(&self, point: ViewportCoord) -> ViewportCoord {
        (
            self.center.0 + (point.0 - 0.5) * self.scale,
            self.center.1 + (point.1 - 0.5) * self.scale,
        )
    }

    /// whether the renderer is completely beyond the viewport
    fn oob(&self) -> bool {
        let (a, b) = self.translate((0.0, 0.0));
        let (c, d) = self.translate((1.0, 1.0));

        a > 1.0 || b > 1.0 || c < 0.0 || d < 0.0
    }

    fn oob_opt(self) -> Option<Self> {
        if self.oob() {
            None
        } else {
            Some(self)
        }
    }
}

impl<'u, R: Renderer> Renderer for ProxyRenderer<'u, R> {
    fn render_body(&mut self, id: BodyId, at: ViewportCoord, scale: (f32, f32)) {
        self.inner.render_body(id, at, scale)
    }
}

fn render<H: Handler>(
    system: &System<H>,
    t: Time,
    viewport: &Viewport,
    renderer: &mut impl Renderer,
) {
    let body = system.tree().get_large_body(viewport.of);
    renderer.render_body(
        body.id().into(),
        viewport.convert_pos(Vector::new(0.0, 0.0)),
        (viewport.dim.0.recip(), viewport.dim.1.recip()),
    );

    for (_, large) in body.large() {
        render_large(large, t, viewport, renderer)
    }
    for (_, small) in body.small() {
        render_small(small, t, viewport, renderer)
    }
}

fn render_large(body: &LargeBody, t: Time, viewport: &Viewport, renderer: &mut impl Renderer) {
    let body_pos = body
        .orbit()
        .as_ref()
        .expect("render_large only accepts large children")
        .position(t);
    renderer.render_body(
        body.id().into(),
        viewport.convert_pos(body_pos),
        (viewport.dim.0.recip(), viewport.dim.1.recip()),
    )
}

fn render_small(body: &SmallBody, t: Time, viewport: &Viewport, renderer: &mut impl Renderer) {
    let body_pos = body.orbit().position(t);
    renderer.render_body(
        body.id().into(),
        viewport.convert_pos(body_pos),
        (viewport.dim.0.recip(), viewport.dim.1.recip()),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_viewport() {
        // TODO
    }
}
