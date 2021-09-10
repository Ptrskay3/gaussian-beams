use pyo3::prelude::*;
use pyo3::types::PyType;
use rayon::prelude::*;
use std::f32::consts::PI;

#[allow(non_snake_case)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct GaussianBeam {
    #[pyo3(get, set)]
    pub wavelength: f64,
    #[pyo3(get, set)]
    pub w_0: f64,
    #[pyo3(get, set)]
    pub z: f64,
    #[pyo3(get, set)]
    pub I_0: f64,
    #[pyo3(get, set)]
    pub y_offset: f64,
    #[pyo3(get, set)]
    pub x_offset: f64,
}

#[pymethods]
impl GaussianBeam {
    #[new]
    pub fn new(wavelength: f64, w_0: f64, z: f64) -> Self {
        Self {
            wavelength,
            w_0,
            z,
            I_0: 1.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    #[classmethod]
    pub fn new_with_offset(
        _cls: &PyType,
        wavelength: f64,
        w_0: f64,
        z: f64,
        x_offset: f64,
        y_offset: f64,
    ) -> PyResult<GaussianBeam> {
        Ok(GaussianBeam {
            wavelength,
            w_0,
            z,
            I_0: 1.0,
            x_offset,
            y_offset,
        })
    }

    pub fn set_x_offset(&mut self, x_offset: f64) {
        self.x_offset = x_offset;
    }

    pub fn set_y_offset(&mut self, y_offset: f64) {
        self.y_offset = y_offset;
    }

    pub fn calc_z_0(&self) -> f64 {
        self.w_0 * self.w_0 * PI as f64 / self.wavelength
    }

    pub fn calc_w_squared_z(&self) -> f64 {
        let z_0 = self.calc_z_0();
        self.w_0 * self.w_0 * (1.0 + (self.z * self.z) / (z_0 * z_0))
    }

    pub fn gouy_phase(&self) -> f64 {
        let z_0 = self.calc_z_0();
        (self.z / z_0).atan()
    }

    #[allow(non_snake_case)]
    pub fn calc_I(&self, x: u32, y: u32) -> f64 {
        let x = x as f64 * 7E-6;
        let y = y as f64 * 7E-6;
        let x_off = self.x_offset * 7E-6;
        let y_off = self.y_offset * 7E-6;
        let rho_squared = ((x - x_off) * (x - x_off) + (y - y_off) * (y - y_off)) as f64;
        self.I_0
            * (self.w_0 * self.w_0 / self.calc_w_squared_z())
            * (-2.0 * rho_squared / self.calc_w_squared_z()).exp()
    }

    pub fn meshgrid(&self, width: u32, height: u32) -> Vec<Vec<f64>> {
        let mut inner = vec![vec![0.0f64; width as usize]; height as usize];
        inner.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                *item = self.calc_I(x as u32, y as u32);
            }
        });
        inner
    }

    pub fn meshgrid2(&self, width: u32, height: u32) -> Vec<Vec<f64>> {
        let mut inner = vec![vec![0.0f64; width as usize]; height as usize];
        for (y, row) in inner.iter_mut().enumerate() {
            row.par_iter_mut().enumerate().for_each(|(x, item)| {
                *item = self.calc_I(x as u32, y as u32);
            })
        }
        inner
    }

    pub fn meshgrid_single(&self, width: u32, height: u32) -> Vec<Vec<f64>> {
        let mut inner = vec![vec![0.0f64; width as usize]; height as usize];
        inner.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, val)| {
                *val = self.calc_I(x as u32, y as u32);
            })
        });
        inner
    }
}
