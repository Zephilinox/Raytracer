
extern crate nalgebra as alg;
extern crate rand;
use rand::prelude::*;

pub fn multiply_colour(a : alg::Vector3<f32>, b : alg::Vector3<f32>) -> alg::Vector3<f32> {
	alg::Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn squared_length(vec : alg::Vector3<f32>) -> f32{
	(vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z)
}

pub fn random_position_in_unit_sphere() -> alg::Vector3<f32> {
	let mut pos : alg::Vector3<f32>;

	loop {
		let mut rng = rand::thread_rng();
		pos = 2.0 as f32 * alg::Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - alg::Vector3::new(1.0, 1.0, 1.0);
		if squared_length(pos) < 1.0 {
			break
		}
	}

	pos
}
