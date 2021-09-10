use crate::beam::GaussianBeam;
use crate::color::CColor;
use image::{DynamicImage, GenericImage};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<GaussianBeam>,
}

impl Scene {
    pub fn new(width: u32, height: u32, elements: &[GaussianBeam]) -> Scene {
        let el = elements.to_vec();
        Scene {
            width,
            height,
            elements: el,
        }
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let mut c = CColor {
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            };
            for element in &scene.elements {
                c += element.calc_I(x, y);
            }
            let color = image::Rgba(c.as_sl());
            img.put_pixel(x, y, color);
        }
    }
    img
}
