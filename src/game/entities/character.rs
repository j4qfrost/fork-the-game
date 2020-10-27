use super::super::components::animate::*;
use super::super::components::sprite::*;
use image::GenericImageView;
use nphysics2d::math::{Isometry, Velocity};
use nphysics2d::object::{DefaultBodyHandle, DefaultBodySet};
use num_traits::{AsPrimitive, FromPrimitive};
use skulpin::skia_safe::{colors, Canvas, Paint, Rect as SkiaRect};
use skulpin::winit::event::ElementState;
use skulpin::winit::event::VirtualKeyCode as Keycode;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CharacterState {
    IdleLeft = 0,
    IdleRight = 1,
    RunningLeft = 2,
    RunningRight = 3,
}

impl FromPrimitive for CharacterState {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::IdleLeft),
            1 => Some(Self::IdleRight),
            2 => Some(Self::RunningLeft),
            3 => Some(Self::RunningRight),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::IdleLeft),
            1 => Some(Self::IdleRight),
            2 => Some(Self::RunningLeft),
            3 => Some(Self::RunningRight),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::IdleLeft),
            1 => Some(Self::IdleRight),
            2 => Some(Self::RunningLeft),
            3 => Some(Self::RunningRight),
            _ => None,
        }
    }
}

impl AsPrimitive<u32> for CharacterState {
    fn as_(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CharacterInput {
    Left,
    Right,
    Interrupt,
}

impl FromPrimitive for CharacterInput {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }
}

/*
#[derive(serde)]
struct CharacterDesc {

}
*/

pub fn draw(canvas: &mut Canvas, isometry: &Isometry<f32>, source: &SpriteSheet, anim: &Animate) {
    let state: CharacterState = anim.state();
    let clip = source.get_clip(state, anim.ticks);

    let img = make_skia_image(&clip.image);

    let position = isometry.translation;
    let paint = Paint::new(colors::RED, None);
    let ratio = clip.width_over_height;

    let rect = SkiaRect::from_xywh(position.x - ratio / 2.0, position.y - 0.5, ratio, 1.0);

    #[cfg(feature = "bounds")]
    {
        use skulpin::skia_safe::Point;
        let p1 = Point::new(position.x - ratio / 2.0, position.y - 0.5);
        let p2 = Point::new(position.x - ratio / 2.0, position.y + 0.5);
        let p3 = Point::new(position.x + ratio / 2.0, position.y + 0.5);
        let p4 = Point::new(position.x + ratio / 2.0, position.y - 0.5);
        canvas.draw_line(p1, p2, &paint);
        canvas.draw_line(p2, p3, &paint);
        canvas.draw_line(p3, p4, &paint);
        canvas.draw_line(p4, p1, &paint);
    }

    canvas.draw_image_rect(img, None, rect, &paint);
}

pub fn delta(state: u32, input: u32) -> u32 {
    let state = CharacterState::from_u32(state).unwrap();
    let input = CharacterInput::from_u32(input).unwrap();
    match (state, input) {
        (_, CharacterInput::Left) => CharacterState::RunningLeft as u32,
        (_, CharacterInput::Right) => CharacterState::RunningRight as u32,
        (CharacterState::RunningLeft, CharacterInput::Interrupt) => CharacterState::IdleLeft as u32,
        (CharacterState::RunningRight, CharacterInput::Interrupt) => {
            CharacterState::IdleRight as u32
        }
        (CharacterState::IdleLeft, CharacterInput::Interrupt) => CharacterState::IdleLeft as u32,
        (CharacterState::IdleRight, CharacterInput::Interrupt) => CharacterState::IdleRight as u32,
    }
}

pub fn animate(
    anim: &mut Animate,
    body_handle: &DefaultBodyHandle,
    bodies: &mut DefaultBodySet<f32>,
) {
    let speed = 2.0;
    let body = bodies.rigid_body_mut(*body_handle).unwrap();
    let (num_states, velocity) = match anim.state() {
        CharacterState::IdleLeft | CharacterState::IdleRight => {
            (4, Velocity::<f32>::linear(0.0, 0.0))
        }
        CharacterState::RunningLeft => (6, Velocity::<f32>::linear(-speed, 0.0)),
        CharacterState::RunningRight => (6, Velocity::<f32>::linear(speed, 0.0)),
    };
    body.set_velocity(velocity);
    anim.ticks = (anim.ticks + 1) % num_states;
}

pub fn source(source_path: String) -> SpriteSheet {
    use std::path::PathBuf;
            let mut display_root = PathBuf::new();
        display_root.push(env!("OUT_DIR"));
        display_root.push("res/assets/adventurer.ron");
    SpriteSheet::from_config(&display_root.to_str().unwrap().to_string());
    let img = image::open(source_path).unwrap();
    let (w, h) = img.dimensions();
    let w = w / 7;
    let h = h / 11;

    let mut clips = HashMap::new();
    // Idle
    let idle_clips = vec![
        Clip::new(&img, Rect{x: 0, y: 0, w, h}, true, true),
        Clip::new(
            &img,
            Rect{x: w, y: 0, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 2, y: 0, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 3, y: 0, w, h},
            true,
            true,
        ),
    ];
    clips.insert(CharacterState::IdleLeft as u32, idle_clips);
    let idle_clips = vec![
        Clip::new(&img, Rect{x: 0, y: 0, w, h}, false, true),
        Clip::new(
            &img,
            Rect{x: w, y: 0, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 2, y: 0, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 3, y: 0, w, h},
            false,
            true,
        ),
    ];
    clips.insert(CharacterState::IdleRight as u32, idle_clips);

    // Crouch
    // let crouch_clips = vec![
    //     Clip::new(&img, (clip_w * 4, 0, clip_w, clip_h), true),
    //     Clip::new(&img, (clip_w * 5, 0, clip_w, clip_h), true),
    //     Clip::new(&img, (clip_w * 6, 0, clip_w, clip_h), true),
    //     Clip::new(&img, (0, clip_h, clip_w, clip_h), true),
    // ];
    // clips.insert("crouch".to_string(), crouch_clips);

    // Running
    let running_clips = vec![
        Clip::new(
            &img,
            Rect{x: w, y: h, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 2, y: h, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 3, y: h, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 4, y: h, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 5, y: h, w, h},
            true,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 6, y: h, w, h},
            true,
            true,
        ),
    ];
    clips.insert(CharacterState::RunningLeft as u32, running_clips);
    let running_clips = vec![
        Clip::new(
            &img,
            Rect{x: w, y: h, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 2, y: h, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 3, y: h, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 4, y: h, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 5, y: h, w, h},
            false,
            true,
        ),
        Clip::new(
            &img,
            Rect{x: w * 6, y: h, w, h},
            false,
            true,
        ),
    ];
    clips.insert(CharacterState::RunningRight as u32, running_clips);

    SpriteSheet::new(clips)
}

pub fn process(
    keycode: Option<Keycode>,
    key_state: &ElementState,
    controlled_character: &mut Animate,
) {
    match (keycode, key_state) {
        (Some(_), ElementState::Released) => {
            controlled_character.delta(CharacterInput::Interrupt as u32);
            controlled_character.ticks = 0;
        }
        (Some(Keycode::Left), _) => controlled_character.delta(CharacterInput::Left as u32),
        (Some(Keycode::Right), _) => controlled_character.delta(CharacterInput::Right as u32),
        _ => {}
    }
}
