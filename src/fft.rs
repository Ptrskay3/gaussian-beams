use rustfft::num_complex;
use rustfft::num_complex::Complex32;
use rustfft::FftPlanner;

pub fn fft(vec: &[f32]) -> Vec<Complex32> {
    let mut v2 = complexify2(vec);
    let len = v2.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);

    fft.process(&mut v2);
    v2
}

pub fn complexify2(buffer: &[f32]) -> Vec<Complex32> {
    buffer
        .iter()
        .map(|&v| num_complex::Complex::<f32>::new(v, 0.0))
        .collect()
}
