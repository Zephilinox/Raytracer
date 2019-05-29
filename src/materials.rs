
extern crate nalgebra as alg;

use ray::*;
use hits::*;
use util::*;

pub struct BounceInfo {
	pub ray : Ray,
	pub attenuation : alg::Vector3<f32>,
	pub absorption : f32
}

pub trait Material {
	fn bounce(&self, ray : &Ray, time_min : f32, time_max : f32, hit_info : &HitInfo, ) -> Option<BounceInfo>;

	fn box_clone(&self) -> Box<Material + Sync>;
}

//https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5

#[derive(Clone)]
pub struct Diffuse {
	pub albedo : alg::Vector3<f32>
}

#[derive(Clone)]
pub struct Metal {
	pub fuzz : f32
}

impl Clone for Box<Material + Sync> {
	fn clone(&self) -> Box<Material + Sync> {
		self.box_clone()
	}
}

impl Diffuse {
	pub fn new () -> Diffuse {
		Diffuse {
			albedo : alg::Vector3::new(0.0, 0.0, 0.0)
		}
	}
}

impl Material for Diffuse {
	fn bounce(&self, ray : &Ray, time_min : f32, time_max : f32, hit_info : &HitInfo) -> Option<BounceInfo> {
		let mut rand_normal = hit_info.normal + random_position_in_unit_sphere();
		rand_normal = alg::normalize(&rand_normal);
		let target = hit_info.pos + rand_normal;

		let intersect_bias = rand_normal * 0.0001;
		let direction = alg::normalize(&(target - hit_info.pos - intersect_bias));
		let out_ray = Ray::new(hit_info.pos + intersect_bias, direction);

		Some(BounceInfo{ray : out_ray, attenuation: hit_info.colour, absorption: 0.9})
	}

	fn box_clone(&self) ->Box<Material + Sync> {
		Box::new((*self).clone())
	}
}

impl Metal {
	pub fn new (fuzz : f32) -> Metal {
		Metal {
			fuzz : fuzz,
		}
	}
}

pub fn reflect(a : alg::Vector3<f32>, b : alg::Vector3<f32>) -> alg::Vector3<f32> {
	a - 2.0 * alg::dot(&a, &b) * b
}

impl Material for Metal {
	fn bounce(&self, ray : &Ray, time_min : f32, time_max : f32, hit_info : &HitInfo) -> Option<BounceInfo> {
		let mut reflected = reflect(alg::normalize(&ray.direction()), hit_info.normal + self.fuzz * random_position_in_unit_sphere());
		reflected = alg::normalize(&reflected);
		let out_ray = Ray::new(hit_info.pos, reflected);
		//if alg::dot(&out_ray.direction(), &hit_info.normal) > 0.001 {
			return Some(BounceInfo{ray : out_ray, attenuation: hit_info.colour, absorption: 0.8});
		//}
		//else {
			//under surface
		//	return None;
		//}
	}

	fn box_clone(&self) ->Box<Material + Sync> {
		Box::new((*self).clone())
	}
}