#![feature(test)]

// use gaussian_beam::beam::GaussianBeam;
// use gaussian_beam::render::{par_render,par_render2, render};
// use gaussian_beam::scene::Scene;
// use image::{DynamicImage, GenericImage};
// use rayon::prelude::*;
// use std::time::Instant;
// use gaussian_beam::fft::{fft, complexify, complexify2};
// use rustfft::FftPlanner;
// use rustfft::num_complex::Complex;

fn main() {
    // let mut planner = FftPlanner::new();
    // let fft = planner.plan_fft_forward(1000000);

    // let mut buffer = vec![Complex{ re: 1.0f32, im: 0.0f32 }; 100000000];
    // let start = Instant::now();
    // fft.process(&mut buffer);
    // let duration = start.elapsed();
    // println!("TIME TOOK: {:?}", duration);
}
//     let beam = GaussianBeam::new_with_offset(632.8 * 10E-9, 1E-3, 0.8, 2500.0, 1200.0);
// //     let beam2 = GaussianBeam::new_with_offset(632.8 * 10E-9, 1E-3, 0.001, 1500.0, 500.0);

// //     let scene = Scene {
// //         width: 3840,
// //         height: 2160,
// //         elements: vec![beam, beam2],
// //     };
//     let start = Instant::now();
//     let _ = beam.meshgrid(5000, 5000);
// //     let img = render(&scene);
//     let duration = start.elapsed();
//     println!("TIME TOOK: {:?}", duration);
// //     img.save("render/img.png").unwrap();
// }

// extern crate test;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_render(b: &mut Bencher) {
//         let beam = GaussianBeam::new_with_offset(632.8 * 10E-9, 1E-3, 0.8, 2500.0, 1200.0);
//         let beam2 = GaussianBeam::new_with_offset(632.8 * 10E-9, 1E-3, 0.001, 1500.0, 500.0);

//         let scene = Scene {
//             width: 3840,
//             height: 2160,
//             elements: vec![beam, beam2],
//         };
//         b.iter(move || {
//             render(&scene);
//         });
//     }
// }
