// use crate::color::CColor;
// use crate::scene::Scene;
// use image::{DynamicImage, GenericImage};
// use rayon::prelude::*;
// use std::sync::{Arc, Mutex};

// pub fn par_render(scene: &Scene) -> DynamicImage {
//     let img = Arc::new(Mutex::new(DynamicImage::new_rgb8(
//         scene.width,
//         scene.height,
//     )));

//     (0..scene.width).into_par_iter().for_each(|x| {
//         (0..scene.height).into_par_iter().for_each(|y| {
//             let img_clone = img.clone();
//             let mut c = CColor {
//                 r: 0,
//                 g: 0,
//                 b: 0,
//                 a: 1,
//             };
//             for element in &scene.elements {
//                 c += element.calc_I(x, y);
//             }
//             let color = image::Rgba(c.as_sl());
//             let mut shared = img_clone.lock().unwrap();
//             shared.put_pixel(x, y, color);
//         });
//     });

//     Arc::try_unwrap(img).unwrap().into_inner().unwrap()
// }

// fn main() {
//     let mut data = vec![0u8; 2000000]; // pretend this is 2 GiB
//     data.chunks_mut(200000) // pretend this is 2 GiB / N threads
//         .par_bridge()
//         .for_each(|d| d.fill(1));
// }

// pub fn par_render2(scene: &Scene) -> DynamicImage {
//     let img = DynamicImage::new_rgb8(scene.width, scene.height);
//     let mut buffer2 = vec![vec![vec![0u8; 4]; scene.width as usize]; scene.height as usize];
//     buffer2
//         .par_iter_mut()
//         .enumerate()
//         .map(|(y, e)| {
//             for x in 0..(scene.width as usize) {
//                 (*e)[x] = get_color(x, y, scene).to_vec();
//             }
//         })
//         .collect::<Vec<_>>();
//     img
// }

// fn par_render3(&scene: &Scene) -> DynamicImage {
//     let img = DynamicImage::new_rgb8(
//         scene.width,
//         scene.height,
//     );
//     img.as_flat_samples_u8().par_iter_mut().enumerate().for_each(|(y, row)| {
//             for (x, item) in row.as_slice().iter().enumerate() {
//                         let mut c = CColor {
//                         r: 0,
//                         g: 0,
//                         b: 0,
//                         a: 1,
//                     };
//                     for element in &scene.elements {
//                         c += element.calc_I(x as u32, y as u32);
//                     }
//                     let color = image::Rgba(c.as_sl());
//                     // let mut it = img.get_pixel_mut(x as u32, y as u32);
//                     *item = c.as_sl()[0].into();
//             }
//         });
//         img
// }

// pub fn get_color(x: usize, y: usize, scene: &Scene) -> [u8; 4] {
//     let mut c = CColor {
//         r: 0,
//         g: 0,
//         b: 0,
//         a: 1,
//     };
//     for element in &scene.elements {
//         c += element.calc_I(x as u32, y as u32);
//     }
//     c.as_sl()
// }

// pub fn render(scene: &Scene) -> DynamicImage {
//     let mut img = DynamicImage::new_rgb8(scene.width, scene.height);

//     for x in 0..scene.width {
//         for y in 0..scene.height {
//             let mut c = CColor {
//                 r: 0,
//                 g: 0,
//                 b: 0,
//                 a: 1,
//             };
//             for element in &scene.elements {
//                 c += element.calc_I(x, y);
//             }
//             let color = image::Rgba(c.as_sl());
//             img.put_pixel(x, y, color);
//         }
//     }
//     img
// }
