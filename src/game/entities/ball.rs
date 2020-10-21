use nphysics2d::math::Isometry;
use skulpin::skia_safe::{colors, Canvas, Paint};

const BALL_RADIUS: f32 = 0.5;

pub fn draw(canvas: &mut Canvas, isometry: &Isometry<f32>) {
    let position = isometry.translation;
    let paint = Paint::new(colors::GREEN, None);

    canvas.draw_circle(
        skulpin::skia_safe::Point::new(position.x, position.y),
        BALL_RADIUS,
        &paint,
    );
}
