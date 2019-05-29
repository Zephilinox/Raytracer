extern crate nalgebra as alg;

pub struct Light {
	pub center: alg::Vector3<f32>,
	pub intensity: f32,
	pub colour: alg::Vector3<f32>,
}

impl Light {
	pub fn new (p_center : alg::Vector3<f32>, p_intensity : f32, p_colour : alg::Vector3<f32>) -> Light {
		Light {
			center: p_center,
			intensity: p_intensity,
			colour: p_colour,
		}
	}
}