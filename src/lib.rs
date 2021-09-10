#![warn(clippy::all)]
#![feature(test)]

pub mod beam;
pub mod color;
pub mod fft;
pub mod scene;

use crate::fft::fft;
use ndarray::Array;
use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArray1};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use rayon::ThreadPoolBuildError;
use rustfft::num_complex::Complex;

pub fn init_par(num_threads: usize) -> Result<(), ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
}

#[pymodule]
fn beams(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<beam::GaussianBeam>()?;

    #[pyfn(m, name = "linspace")]
    fn py_linspace(py: Python, start: f64, stop: f64, num: usize) -> PyResult<Py<PyArray1<f64>>> {
        let array = Array::linspace(start, stop, num);

        Ok(array.into_pyarray(py).to_owned())
    }

    #[pyfn(m, name = "fft")]
    fn py_fft(py: Python, array: Vec<f32>) -> PyResult<Py<PyArray1<Complex<f32>>>> {
        let array = fft(&array);

        Ok(array.into_pyarray(py).to_owned())
    }

    fn axpy(a: f64, x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
        a * &x + &y
    }

    fn mult(a: f64, mut x: ArrayViewMutD<'_, f64>) {
        x *= a;
    }

    #[pyfn(m, name = "axpy")]
    fn axpy_py<'py>(
        py: Python<'py>,
        a: f64,
        x: PyReadonlyArrayDyn<f64>,
        y: PyReadonlyArrayDyn<f64>,
    ) -> &'py PyArrayDyn<f64> {
        let x = x.as_array();
        let y = y.as_array();
        axpy(a, x, y).into_pyarray(py)
    }

    // wrapper of `mult`
    #[pyfn(m, name = "mult")]
    fn mult_py(_py: Python<'_>, a: f64, x: &PyArrayDyn<f64>) -> PyResult<()> {
        let x = unsafe { x.as_array_mut() };
        mult(a, x);
        Ok(())
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::beam::GaussianBeam;
    use crate::scene::{render, Scene};
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_render(b: &mut Bencher) {
        let mut beam = GaussianBeam::new(632.8 * 10E-9, 1E-3, 0.8);
        beam.set_x_offset(200.0);
        beam.set_y_offset(200.0);
        let mut beam2 = GaussianBeam::new(632.8 * 10E-9, 1E-3, 0.001);
        beam2.set_x_offset(100.0);
        beam2.set_y_offset(100.0);

        let scene = Scene {
            width: 500,
            height: 500,
            elements: vec![beam, beam2],
        };
        b.iter(move || {
            render(&scene);
        });
    }
}
