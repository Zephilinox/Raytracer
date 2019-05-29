
extern crate nalgebra as alg;

#[derive(Copy, Clone)]
pub struct Ray {
	pub a: alg::Vector3<f32>,
	pub b: alg::Vector3<f32>,
}

impl Ray {
	pub fn new(a: alg::Vector3<f32>,
		b: alg::Vector3<f32>) -> Ray {
		Ray {
			a: a,
			b: b,
		}
	}

	pub fn origin(&self) -> alg::Vector3<f32> {
		self.a
	}

	pub fn direction(&self) -> alg::Vector3<f32> {
		self.b
	}

	pub fn point_at_parameter(&self, time: f32) -> alg::Vector3<f32> {
		self.a + (time * self.b)
	}
}