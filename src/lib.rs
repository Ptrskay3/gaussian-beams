pub mod beam;
pub mod color;
pub mod fft;
pub mod render;
pub mod scene;

use crate::fft::fft;
use ndarray::Array;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;
use rayon::ThreadPoolBuildError;
use rustfft::num_complex::Complex;
use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

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
