use super::animate::Animate;
use image::{DynamicImage, GenericImageView, Rgba};
use nphysics2d::math::Isometry;
use num_traits::AsPrimitive;
use skulpin::skia_safe::{
    AlphaType, Canvas, ColorInfo, ColorSpace, ColorType, Data, IRect, ISize, Image, ImageInfo,
};
use std::collections::HashMap;

type DrawFunction = fn(&mut Canvas, &Isometry<f32>, &SpriteSheet, &Animate) -> ();

pub struct Sprite {
    pub draw_fn: DrawFunction,
    pub source: SpriteSheet,
}

impl Sprite {
    pub fn new(draw_fn: DrawFunction, source: SpriteSheet) -> Self {
        Self { draw_fn, source }
    }
}

pub struct SpriteSheet {
    clips: HashMap<u32, Vec<Clip>>,
}

impl SpriteSheet {
    pub fn new(clips: HashMap<u32, Vec<Clip>>) -> Self {
        Self { clips }
    }
}

impl SpriteSheet {
    #[inline]
    pub fn get_clip<T: AsPrimitive<u32>>(&self, key: T, it: usize) -> &Clip {
        &self.clips.get(&(key.as_())).unwrap()[it]
    }
}

#[derive(Clone)]
pub struct Clip {
    pub image: DynamicImage,
    pub width_over_height: f32,
}

impl Clip {
    pub fn new(source: &DynamicImage, rect: &IRect, is_flipped: bool, squeeze: bool) -> Self {
        let mut cropped = source.crop_imm(
            rect.x() as u32,
            rect.y() as u32,
            rect.width() as u32,
            rect.height() as u32,
        );

        if is_flipped {
            cropped = cropped.fliph();
        }

        if squeeze {
            Clip::squeeze(&mut cropped);
        }

        let image = cropped.flipv();
        let width_over_height = image.width() as f32 / image.height() as f32;
        Self {
            image,
            width_over_height,
        }
    }

    fn squeeze(source: &mut DynamicImage) {
        for _ in 0..4 {
            let rgba_img = source.as_rgba8().unwrap();
            for (i, mut row) in rgba_img.enumerate_rows() {
                if row.any(|p| p.2 != &Rgba::from([0, 0, 0, 0])) {
                    *source = source.crop_imm(0, i, source.width(), source.height() - i);
                    break;
                }
            }
            *source = source.rotate90();
        }
    }
}

pub fn make_skia_image(img: &DynamicImage) -> Image {
    let (w, h) = img.dimensions();
    let bytes = img.to_bytes();
    let data = unsafe { Data::new_bytes(&bytes) };

    let color_info = ColorInfo::new(
        ColorType::RGBA8888,
        AlphaType::Opaque,
        ColorSpace::new_srgb(),
    );
    let size = ISize::new(w as i32, h as i32);
    let img_info = ImageInfo::from_color_info(size, color_info);
    Image::from_raster_data(&img_info, data, w as usize * img_info.bytes_per_pixel()).unwrap()
}
