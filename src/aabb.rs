use ray::*;
use util::*;
extern crate nalgebra as alg;

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
pub struct AABB {
    min: alg::Vector3<f32>,
    max: alg::Vector3<f32>,
}

fn mymin(a: f32, b: f32) -> f32 {
    if a < b{
        a
    } else {
        b
    }
}

fn mymax(a: f32, b: f32) -> f32 {
    if a > b{
        a
    } else {
        b
    }
}

impl AABB {
    pub fn new(a: alg::Vector3<f32>, b: alg::Vector3<f32>) -> AABB {
        AABB {
            min: a,
            max: b,
        }
    }

    pub fn zero() -> AABB {
        AABB::new(alg::Vector3::new(0.0, 0.0, 0.0), alg::Vector3::new(0.0, 0.0, 0.0))
    }

    pub fn min(&self) -> alg::Vector3<f32> { self.min }
    pub fn max(&self) -> alg::Vector3<f32> { self.max }

    pub fn longest_axis(&self) -> Axis {
        let a = self.max.x - self.min.x;
        let b = self.max.y - self.min.y;
        let c = self.max.z - self.min.z;
        if a > b && a > c {
            return Axis::X;
        } else if b > c {
            return Axis::Y;
        } else {
            return Axis::Z;
        }
    }

    pub fn area(&self) -> f32 {
        let a = self.max.x - self.min.x;
        let b = self.max.y - self.min.y;
        let c = self.max.z - self.min.z;
        return 2.0 * (a*b + b*c + c*a);
    }

    /// Check if the given ray hits the bounding box.
    pub fn hit(&self, r: &Ray, time_min: f32,  time_max: f32) -> bool {
        let mut t0 = mymin((self.min.x - r.origin().x) / r.direction().x,
                        (self.max.x - r.origin().x) / r.direction().x);
        let mut t1 = mymax((self.min.x - r.origin().x) / r.direction().x,
                        (self.max.x - r.origin().x) / r.direction().x);
        let mut tmin = mymax(t0, time_min);
        let mut tmax = mymin(t1, time_max);
        if tmax <= tmin {
            return false;
        }

        t0 = mymin((self.min.y - r.origin().y) / r.direction().y,
                        (self.max.y - r.origin().y) / r.direction().y);
        t1 = mymax((self.min.y - r.origin().y) / r.direction().y,
                        (self.max.y - r.origin().y) / r.direction().y);
        tmin = mymax(t0, time_min);
        tmax = mymin(t1, time_max);
        if tmax <= tmin {
            return false;
        }

        t0 = mymin((self.min.z - r.origin().z) / r.direction().z,
                        (self.max.z - r.origin().z) / r.direction().z);
        t1 = mymax((self.min.z - r.origin().z) / r.direction().z,
                        (self.max.z - r.origin().z) / r.direction().z);
        tmin = mymax(t0, time_min);
        tmax = mymin(t1, time_max);
        if tmax <= tmin {
            return false;
        }

        return true;
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = alg::Vector3::new(box0.min().x.min(box1.min().x),
                          box0.min().y.min(box1.min().y),
                          box0.min().z.min(box1.min().z));
    let big = alg::Vector3::new(box0.max().x.max(box1.max().x),
                        box0.max().y.max(box1.max().y),
                        box0.max().z.max(box1.max().z));
    return AABB::new(small, big);
}
