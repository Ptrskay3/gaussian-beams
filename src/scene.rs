use crate::beam::GaussianBeam;

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
