use hits::*;
use aabb::*;
use ray::*;
extern crate nalgebra as alg;
use std::f32;
use std::cmp::Ordering;

pub struct BVHNode {
    left: Box<Hitable + Sync>,
    right: Box<Hitable + Sync>,
    bbox: AABB,
}

fn compare_x(a: &Box<Hitable + Sync>, b: &Box<Hitable + Sync>) -> Ordering {
    let l = a.bounding_box(0.0, 0.0).unwrap();
    let r = b.bounding_box(0.0, 0.0).unwrap();
    if l.min().x < r.min().x {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn compare_y(a: &Box<Hitable + Sync>, b: &Box<Hitable + Sync>) -> Ordering {
    let l = a.bounding_box(0.0, 0.0).unwrap();
    let r = b.bounding_box(0.0, 0.0).unwrap();
    if l.min().y < r.min().y {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn compare_z(a: &Box<Hitable + Sync>, b: &Box<Hitable + Sync>) -> Ordering {
    let l = a.bounding_box(0.0, 0.0).unwrap();
    let r = b.bounding_box(0.0, 0.0).unwrap();
    if l.min().z < r.min().z {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

impl BVHNode {
    pub fn new(mut list: Vec<Box<Hitable + Sync>>, time_min: f32, time_max: f32) -> BVHNode
    {
        let length = list.len();
        assert_ne!(length, 0);

        //Construct a bounding box that contains every node in the list
        let box1 = list.iter().fold(AABB::zero(), |bbox, ref x| {
            let box2 = x.bounding_box(time_min, time_max).unwrap();
            surrounding_box(&box2, &bbox)
        });

        //Sort the list of nodes based on the longest axis of the surrounding bounding box
        let axis = box1.longest_axis();
        match axis {
            Axis::X => list.sort_by(compare_x),
            Axis::Y => list.sort_by(compare_y),
            Axis::Z => list.sort_by(compare_z),
        }

        let boxes: Vec<AABB> = list.iter().map(|h| h.bounding_box(time_min, time_max).unwrap()).collect();
        let mut la : Vec<f32> = Vec::with_capacity(length);
        let mut ra : Vec<f32> = Vec::with_capacity(length);

        //Create the surface area for each box on the left
        la.push(boxes[0].area());
        let mut left_box = boxes[0].clone();
        for i in 1..length-1 {
            left_box = surrounding_box(&left_box, &boxes[i]);
            la.push(left_box.area());
        }

        //Now do it in reverse order for the right half
        ra.push(boxes[length-1].area());
        let mut right_box = boxes[length-1].clone();
        for i in (1..length-1).rev() {
            right_box = surrounding_box(&right_box, &boxes[i]);
            ra.push(right_box.area());
        }
        ra.push(0.0);
        ra.reverse();

        //Find the split point by using the minimum of children*surface area
        let mut min_sah = f32::MAX;
        let mut min_sah_i = 0;
        for i in 0..length-1 {
            let sah = i as f32 * la[i] + (length-i-1) as f32 * ra[i+1];
            if sah < min_sah {
                min_sah_i = i;
                min_sah = sah;
            }
        }

        let mut rest = list.split_off(min_sah_i+1);
        
        let left;
        if min_sah_i == 0 {
            left = list.remove(0);
        } else {
            left = Box::new(BVHNode::new(list, time_min, time_max));
        }
        
        let right;
        if rest.len() == 1 {
            right = rest.remove(0);
        } else {
            right = Box::new(BVHNode::new(rest, time_min, time_max));
        }

        BVHNode{
            left: left,
            right: right,
            bbox: box1,
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, time_min: f32, time_max: f32) -> Option<HitInfo> {
        if self.bbox.hit(r, time_min, time_max) {
            let hitl = self.left.hit(r, time_min, time_max);
            let hitr = self.right.hit(r, time_min, time_max);
            match (hitl, hitr) {

                (Some(left), Some(right)) => {
                    if left.time < right.time {
                        return Some(left);
                    } else {
                        return Some(right);
                    }
                },

                (Some(left), None) => {
                    return Some(left);
                },

                (None, Some(right)) => {
                    return Some(right);
                },

                _ => {
                    return None;
                }
            }
        }

        return None;
    }

    fn bounding_box(&self, _time_min: f32, _time_max: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}