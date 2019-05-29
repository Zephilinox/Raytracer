
extern crate nalgebra as alg;

use aabb::*;
use ray::*;
use shapes::*;
use materials::*;

pub struct HitInfo {
	pub time : f32,
	pub pos : alg::Vector3<f32>,
	pub normal : alg::Vector3<f32>,
	pub colour : alg::Vector3<f32>,
	pub material: Box<Material + Sync>
}

pub trait Hitable {
	fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitInfo>;
	fn bounding_box(&self, time_min: f32, time_max: f32) -> Option<AABB>;
}

impl Hitable for Sphere {
	fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitInfo> {
		let oc = ray.origin() - self.center;
		let a = ray.direction().dot(&ray.direction());
		let b = oc.dot(&ray.direction());
		let c = oc.dot(&oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;

		if discriminant > 0.0 {
			let mut hit_info = HitInfo {time: 0.0, pos: alg::Vector3::new(0.0, 0.0, 0.0), normal: alg::Vector3::new(0.0, 0.0, 0.0), colour : self.colour, material : self.material.clone()};

			let mut temp = (-b - discriminant.sqrt()) / a;
			if temp < time_max && temp > time_min {
				hit_info.time = temp;
				hit_info.pos = ray.point_at_parameter(temp);
				hit_info.normal = (hit_info.pos - self.center) / self.radius;
				return Some(hit_info);
			}

			temp = (-b + discriminant.sqrt()) / a;
			if temp < time_max && temp > time_min {
				hit_info.time = temp;
				hit_info.pos = ray.point_at_parameter(temp);
				hit_info.normal = (hit_info.pos - self.center) / self.radius;
				return Some(hit_info);
			}
		}

		None
	}

	fn bounding_box(&self, _time_min: f32, _time_max: f32) -> Option<AABB> {
		Some(AABB::new(self.center - alg::Vector3::new(self.radius, self.radius, self.radius),
		self.center + alg::Vector3::new(self.radius, self.radius, self.radius)))
	}
}

impl Hitable for Cube {
	fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitInfo> {
		let oc = ray.origin() - self.center;
		let mut hit_info = HitInfo {time: 0.0, pos: alg::Vector3::new(0.0, 0.0, 0.0), normal: alg::Vector3::new(1.0, 0.0, 0.0), colour : self.colour, material : self.material.clone()};

		let mut min = alg::Vector3::new(
		-self.extents,
		-self.extents,
		-self.extents);
		
		let mut max = alg::Vector3::new(
		self.extents,
		self.extents,
		self.extents);

		min = min + self.center;
		max = max + self.center;

		let mut min_x = (min.x - oc.x) / ray.direction().x;
		let mut max_x = (max.x - oc.x) / ray.direction().x;

		let mut min_y = (min.y - oc.y) / ray.direction().y;
		let mut max_y = (max.y - oc.y) / ray.direction().y;

		let mut min_z = (min.z - oc.z) / ray.direction().z;
		let mut max_z = (max.z - oc.z) / ray.direction().z;

		let mut min_t : f32 = time_min;
		let mut max_t : f32 = time_max;

		let mut near_index = 0;
		let mut far_index = 0;

		if ray.direction().x != 0.0 {
			min_t = min_t.max(min_x.min(max_x));
			max_t = max_t.min(min_x.max(max_x));
		}

		if ray.direction().y != 0.0 {
			if min_y.min(max_y) > min_t {
				near_index = 1;
			}

			min_t = min_t.max(min_y.min(max_y));

			if max_t > min_y.max(max_y) {
				far_index = 1;
			}

			max_t = max_t.min(min_y.max(max_y));
		}

		if ray.direction().z != 0.0 {
			if min_z.min(max_z) > min_t {
				near_index = 2;
			}

			min_t = min_t.max(min_z.min(max_z));

			if max_t > min_z.max(max_z) {
				far_index = 2;
			}
			
			max_t = max_t.min(min_z.max(max_z));
		}

		if max_t < 0.0 || max_t < min_t {
			//didn't intersect
			return None;
		}

		let normals : [alg::Vector3<f32>; 3] = [alg::Vector3::new(1.0, 0.0, 0.0), alg::Vector3::new(0.0, 1.0, 0.0), alg::Vector3::new(0.0, 0.0, 1.0)];
		let index : usize;
		let t : f32;
		if min_t < 0.0 {
			//inside self
			t = max_t;
			index = far_index;
		}
		else {
			t = min_t;
			index = near_index;
		}

		if t < time_min || t > time_max {
			//didn't intersect
			return None;
			println!(":O")
		}

		hit_info.time = t;
		hit_info.pos = ray.point_at_parameter(t);
		hit_info.normal = normals[index];

		if alg::dot(&hit_info.normal, &oc) < 0.0 {
			hit_info.normal = hit_info.normal * -1.0;
		}

		hit_info.normal = alg::normalize(&hit_info.normal);

		/*let mut near_index = 0;
		let mut far_index = 0;

		if min_x > max_x {
			let temp = min_x;
			min_x = max_x;
			max_x = temp;
		}

		if min_y > max_y {
			let temp = min_y;
			min_y = max_y;
			max_y = temp;
		}

		if min_x > max_y || min_y > max_x {
			return None;
		}

		if min_y > min_x {
			min_x = min_y;
			near_index = 0;
		}

		if max_x > max_y {
			max_x = max_y;
			far_index = 0;
		}

		if min_z > max_z {
			let temp = min_z;
			min_z = max_z;
			max_z = temp;
		}

		if min_x > max_z || min_z > max_x {
			return None;
		}

		if min_z > min_x {
			min_x = min_z;
			near_index = 2;
		}

		if max_x > max_z {
			max_x = max_z;
			far_index = 2;
		}

		if min_x > max_x {
			return None;
		}

		let normals : [alg::Vector3<f32>; 3] = [alg::Vector3::new(-1.0, 0.0, 0.0), alg::Vector3::new(0.0, -1.0, 0.0), alg::Vector3::new(0.0, 0.0, -1.0)];
		
		if min_x < 0.0 {
			hit_info.time = max_x;
			hit_info.pos = ray.point_at_parameter(max_x);
			hit_info.normal = normals[far_index];
		}
		else
		{
			hit_info.time = min_x;
			hit_info.pos = ray.point_at_parameter(min_x);
			hit_info.normal = normals[near_index];
		}

		if alg::dot(&hit_info.normal, &oc) < 0.0 {
			hit_info.normal = -hit_info.normal;
		}

		hit_info.normal = alg::normalize(&hit_info.normal);	*/

		Some(hit_info)
	}

	fn bounding_box(&self, _time_min: f32, _time_max: f32) -> Option<AABB> {
		//Some(AABB::new(alg::Vector3::new(0.0, 0.0, 0.0), alg::Vector3::new(0.0, 0.0, 0.0)))
		Some(AABB::new(self.center - alg::Vector3::new(self.extents, self.extents, self.extents),
		self.center + alg::Vector3::new(self.extents, self.extents, self.extents)))
	}
}

impl Hitable for Triangle {
	fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitInfo> {
		let mut hit_info = HitInfo {time: 0.0, pos: alg::Vector3::new(0.0, 0.0, 0.0), normal: alg::Vector3::new(1.0, 0.0, 0.0), colour : self.colour, material : self.material.clone()};
		
		let mut edge1 = [0.0; 3];
		edge1[0] = self.vertices[1].x - self.vertices[0].x;
		edge1[1] = self.vertices[1].y - self.vertices[0].y;
		edge1[2] = self.vertices[1].z - self.vertices[0].z;

		let mut edge2 = [0.0; 3];
		edge2[0] = self.vertices[2].x - self.vertices[0].x;
		edge2[1] = self.vertices[2].y - self.vertices[0].y;
		edge2[2] = self.vertices[2].z - self.vertices[0].z;

		let mut normal = [0.0; 3];
		normal[0] = (edge1[1] * edge2[2]) - (edge1[2] * edge2[1]);
		normal[1] = (edge1[2] * edge2[0]) - (edge1[0] * edge2[2]);
		normal[2] = (edge1[0] * edge2[1]) - (edge1[1] * edge2[0]);

		let magnitude : f32 = (normal[0] * normal[0]) + (normal[1] * normal[1]) + (normal[2] * normal[2]).sqrt();
		normal[0] = normal[0] / magnitude;
		normal[1] = normal[1] / magnitude;
		normal[2] = normal[2] / magnitude;

		let denominator = (normal[0] * ray.direction().x) + (normal[1] * ray.direction().y) + (normal[2] * ray.direction().z);

		if denominator.abs() < 0.1
		{
			return None;
		}

		let distance = (-normal[0] * self.vertices[0].x) + (-normal[1] * self.vertices[0].y) + (-normal[2] * self.vertices[0].z);
		let numerator = -1.0 * (((normal[0] * ray.a.x) + (normal[1] * ray.a.x) + (normal[2] * ray.a.z)) + distance);
		let t = numerator / denominator;

		let mut e1 = [0.0; 3];
		e1[0] = self.vertices[1].x - self.vertices[0].x;
		e1[1] = self.vertices[1].y - self.vertices[0].y;
		e1[2] = self.vertices[1].z - self.vertices[0].z;
		
		let mut e2 = [0.0; 3];
		e2[0] = self.vertices[2].x - self.vertices[1].x;
		e2[1] = self.vertices[2].y - self.vertices[1].y;
		e2[2] = self.vertices[2].z - self.vertices[1].z;
		
		let mut e3 = [0.0; 3];
		e3[0] = self.vertices[0].x - self.vertices[2].x;
		e3[1] = self.vertices[0].y - self.vertices[2].y;
		e3[2] = self.vertices[0].z - self.vertices[2].z;

		let mut edge_normals = [0.0; 3];
		edge_normals[0] = (e1[1] * normal[2]) - (e1[2] * normal[1]);
		edge_normals[1] = (e1[2] * normal[0]) - (e1[0] * normal[2]);
		edge_normals[2] = (e1[0] * normal[1]) - (e1[1] * normal[0]);

		let mut temp = [0.0; 3];
		temp[0] = ray.point_at_parameter(t).x - self.vertices[0].x;
		temp[1] = ray.point_at_parameter(t).y - self.vertices[0].y;
		temp[2] = ray.point_at_parameter(t).z - self.vertices[0].z;

		let mut determinant = (edge_normals[0] * temp[0]) + (edge_normals[1] * temp[1]) + (edge_normals[2] * temp[2]);

		if determinant > 0.000001
		{
			return None;
		}
		
		edge_normals[0] = (e2[1] * normal[2]) - (e2[2] * normal[1]);
		edge_normals[1] = (e2[2] * normal[0]) - (e2[0] * normal[2]);
		edge_normals[2] = (e2[0] * normal[1]) - (e2[1] * normal[0]);

		temp[0] = ray.point_at_parameter(t).x - self.vertices[1].x;
		temp[1] = ray.point_at_parameter(t).y - self.vertices[1].y;
		temp[2] = ray.point_at_parameter(t).z - self.vertices[1].z;

		determinant = (edge_normals[0] * temp[0]) + (edge_normals[1] * temp[1]) + (edge_normals[2] * temp[2]);

		if determinant > 0.000001
		{
			return None;
		}

		edge_normals[0] = (e3[1] * normal[2]) - (e3[2] * normal[1]);
		edge_normals[1] = (e3[2] * normal[0]) - (e3[0] * normal[2]);
		edge_normals[2] = (e3[0] * normal[1]) - (e3[1] * normal[0]);

		temp[0] = ray.point_at_parameter(t).x - self.vertices[2].x;
		temp[1] = ray.point_at_parameter(t).y - self.vertices[2].y;
		temp[2] = ray.point_at_parameter(t).z - self.vertices[2].z;

		determinant = (edge_normals[0] * temp[0]) + (edge_normals[1] * temp[1]) + (edge_normals[2] * temp[2]);

		if determinant > 0.000001
		{
			return None;
		}

		hit_info.time = t;
		hit_info.pos = ray.point_at_parameter(t);
		if self.normal == alg::Vector3::new(0.0, 0.0, 0.0)
		{
			/*
				let mut edge1 = [0.0; 3];
				edge1[0] = self.vertices[0].x - self.vertices[2].x;
				edge1[1] = self.vertices[0].y - self.vertices[2].y;
				edge1[2] = self.vertices[0].z - self.vertices[2].z;

				let mut edge2 = [0.0; 3];
				edge2[0] = self.vertices[2].x - self.vertices[1].x;
				edge2[1] = self.vertices[2].y - self.vertices[1].y;
				edge2[2] = self.vertices[2].z - self.vertices[1].z;
				
				// Calculate the cross product of those two vectors to get the un-normalized value for this face normal.
				normals[index].x = (vector1[1] * vector2[2]) - (vector1[2] * vector2[1]);
				normals[index].y = (vector1[2] * vector2[0]) - (vector1[0] * vector2[2]);
				normals[index].z = (vector1[0] * vector2[1]) - (vector1[1] * vector2[0]);

				// Calculate the length.
				length = (float)sqrt((normals[index].x * normals[index].x) + (normals[index].y * normals[index].y) + 
									(normals[index].z * normals[index].z));

				// Normalize the final value for this face using the length.
				normals[index].x = (normals[index].x / length);
				normals[index].y = (normals[index].y / length);
				normals[index].z = (normals[index].z / length);
			*/
			hit_info.normal.x = normal[0];
			hit_info.normal.y = normal[1];
			hit_info.normal.z = normal[2];
		}
		else {
			hit_info.normal = self.normal;
		}
		//hit_info.normal = alg::Vector3::new(1.0, 0.0, 0.0);
		Some(hit_info)
	}

	fn bounding_box(&self, _time_min: f32, _time_max: f32) -> Option<AABB> {
		//https://stackoverflow.com/questions/39974191/triangle-bounding-box

		let min_x = self.vertices[0].x.min(self.vertices[1].x.min(self.vertices[2].x));
		let max_x = self.vertices[0].x.max(self.vertices[1].x.max(self.vertices[2].x));

		let min_y = self.vertices[0].y.min(self.vertices[1].y.min(self.vertices[2].y));
		let max_y = self.vertices[0].y.max(self.vertices[1].y.max(self.vertices[2].y));

		let min_z = self.vertices[0].z.min(self.vertices[1].z.min(self.vertices[2].z));
		let max_z = self.vertices[0].z.max(self.vertices[1].z.max(self.vertices[2].z));

		return Some(AABB::new(alg::Vector3::new(min_x, min_y, min_z), alg::Vector3::new(max_x, max_y, max_z)));
	}
}