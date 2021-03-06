use super::components::animate::Animate;
use super::components::input::KeyInputHandler;
use super::components::primitive::Primitive;
use super::components::sprite::Sprite;
use super::entities::ball;
use super::entities::character;
use legion::{Resources, World};
use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground,
    RigidBodyDesc,
};

pub struct Level {
    name: String,
}

pub const GROUND_THICKNESS: f32 = 0.2;
pub const GROUND_HALF_EXTENTS_WIDTH: f32 = 3.0;
pub const BALL_RADIUS: f32 = 0.5;
pub const BALL_COUNT: usize = 5;

impl Level {
    pub fn new() -> Self {
        Self {
            name: "test".to_string(),
        }
    }

    pub fn init(&self, world: &mut World, resources: &mut Resources) {
        println!("Loading level {:?}", self.name);
        // A rectangle that the balls will fall on
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            GROUND_HALF_EXTENTS_WIDTH,
            GROUND_THICKNESS,
        )));

        let mut bodies = resources
            .get_mut::<DefaultBodySet<f32>>()
            .unwrap_or_else(|| panic!("{:?}- Bodyset", self.name));
        let mut colliders = resources
            .get_mut::<DefaultColliderSet<f32>>()
            .unwrap_or_else(|| panic!("{:?}- Colliderset", self.name));

        // Build a static ground body and add it to the body set.
        let ground_body_handle = bodies.insert(Ground::new());

        // Build the collider.
        let ground_collider = ColliderDesc::new(ground_shape)
            .translation(Vector2::y() * -GROUND_THICKNESS)
            .build(BodyPartHandle(ground_body_handle, 0));

        // Add the collider to the collider set.
        colliders.insert(ground_collider);

        let ball_shape_handle = ShapeHandle::new(Ball::new(BALL_RADIUS));

        let shift = BALL_RADIUS + ColliderDesc::<f32>::default_margin();
        let centerx = shift * (BALL_COUNT as f32) / 2.0;

        let x = shift - centerx;
        let y = shift;

        // Build the rigid body.
        let rigid_body = RigidBodyDesc::new()
            .translation(Vector2::new(x, y))
            .status(BodyStatus::Static)
            .build();

        // Insert the rigid body to the body set.
        let rigid_body_handle = bodies.insert(rigid_body);

        // Build the collider.
        let ball_collider = ColliderDesc::new(ball_shape_handle.clone())
            .density(1.0)
            .sensor(true)
            .build(BodyPartHandle(rigid_body_handle, 0));

        // Insert the collider to the body set.
        colliders.insert(ball_collider);
        let primitive = Primitive::new(ball::draw);
        world.push((rigid_body_handle, primitive));

        let source = character::source();
        let clip = source.get_clip(character::CharacterState::IdleLeft, 0);
        let ratio = clip.width_over_height;
        let sprite = Sprite::new(character::draw, source);
        let animate = Animate::new(0, character::delta, character::animate);
        // Build the rigid body.
        let rigid_body = RigidBodyDesc::new().translation(Vector2::y()).build();

        // Insert the rigid body to the body set.
        let rigid_body_handle = bodies.insert(rigid_body);

        let box_shape_handle = ShapeHandle::new(Cuboid::new(Vector2::new(ratio / 2.0, 0.5)));

        // Build the collider.
        let box_collider = ColliderDesc::new(box_shape_handle)
            .density(1.0)
            .build(BodyPartHandle(rigid_body_handle, 0));

        // Insert the collider to the body set.
        colliders.insert(box_collider);

        let key_input_handler = KeyInputHandler::new(character::process);

        world.push((rigid_body_handle, key_input_handler, animate, sprite));
    }
}
