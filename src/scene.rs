
extern crate nalgebra as alg;

use hits::*;
use ray::*;
use aabb::*;

pub struct Scene<'a> {
	pub hitables : Vec<Box<Hitable + Sync + 'a>>
}

impl<'a> Scene<'a> {
	pub fn new() -> Scene<'a> {
		Scene { hitables : Vec::new() }
	}

	pub fn add<T : Hitable + Sync + 'a>(&mut self, hitable : T) {
		self.hitables.push(Box::new(hitable));
	}
}

impl<'a> Hitable for Scene<'a> {
	fn hit(&self, ray: &Ray, time_min : f32, time_max : f32) -> Option<HitInfo> {
		let mut closest = time_max;
		let mut temp_info : Option<HitInfo> = None;

		for hitable in &self.hitables {
			if let Some(hit) = hitable.hit(ray, time_min, closest) {
				if hit.time < closest {
					closest = hit.time;
					temp_info = Some(hit);
				}
			}
		}

		temp_info
	}

	fn bounding_box(&self, time_min: f32, time_max: f32) -> Option<AABB> {
        if self.hitables.len() == 0 {
            return None;
        }
        let bb1 = self.hitables[0].bounding_box(time_min, time_max);
        if let Some(bb1) = bb1 {
            let mut bb = bb1;
            for h in self.hitables.iter().skip(1) {
                if let Some(temp_box) = h.bounding_box(time_min, time_max) {
                    bb = surrounding_box(&bb, &temp_box);
                } else {
                    // One of our items is infinite.
                    return None;
                }
            }
            return Some(bb);
        } else {
            return None;
        }
	}
}