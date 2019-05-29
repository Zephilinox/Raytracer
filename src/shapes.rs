
extern crate nalgebra as alg;

use materials::*;

pub struct Sphere {
	pub center: alg::Vector3<f32>,
	pub radius: f32,
	pub colour: alg::Vector3<f32>,
	pub material: Box<Material + Sync>
}

impl Sphere {
	pub fn new (p_center : alg::Vector3<f32>, p_radius : f32, p_colour : alg::Vector3<f32>, p_material : Box<Material + Sync>) -> Sphere {
		Sphere {
			center: p_center,
			radius: p_radius,
			colour: p_colour,
			material: p_material
		}
	}
}

pub struct Cube {
	pub center: alg::Vector3<f32>,
	pub extents: f32,
	pub colour: alg::Vector3<f32>,
	pub material: Box<Material + Sync>
}

impl Cube {
	pub fn new (p_center : alg::Vector3<f32>, p_extents : f32, p_colour : alg::Vector3<f32>, p_material : Box<Material + Sync>) -> Cube {
		Cube {
			center: p_center,
			extents: p_extents,
			colour: p_colour,
			material: p_material
		}
	}
}

pub struct Triangle {
	pub vertices: [alg::Vector3<f32>; 3],
	pub normal: alg::Vector3<f32>,
	pub colour: alg::Vector3<f32>,
	pub material: Box<Material + Sync>
}

impl Triangle {
	pub fn new (p_vertices : [alg::Vector3<f32>; 3], p_normal : alg::Vector3<f32>, p_colour : alg::Vector3<f32>, p_material : Box<Material + Sync>) -> Triangle {
		Triangle {
			vertices: p_vertices,
			normal: p_normal,
			colour: p_colour,
			material: p_material
		}
	}
}