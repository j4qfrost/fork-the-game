use nphysics2d::math::Isometry;
use skulpin::skia_safe::Canvas;

type DrawFunction = fn(&mut Canvas, &Isometry<f32>) -> ();

pub struct Primitive {
    pub draw_fn: DrawFunction,
}

impl Primitive {
    pub fn new(draw_fn: DrawFunction) -> Self {
        Self { draw_fn }
    }
}
