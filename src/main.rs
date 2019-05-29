use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Write};

extern crate nalgebra as alg;
extern crate rand;
extern crate rayon;
extern crate image;
extern crate tobj;

use rand::*;
use rayon::prelude::*;
use image::*;
use tobj::*;

mod ray;
mod shapes;
mod hits;
mod scene;
mod materials;
mod util;
mod aabb;
mod bvh;
mod light;

use ray::*;
use shapes::*;
use shapes::Triangle;
use hits::*;
use scene::*;
use materials::*;
use util::*;
use bvh::*;
use light::*;

static DEBUG_NORMALS : bool = false;
static USE_BVH : bool = false;
static USE_LIGHTS : bool = true;

pub fn colour(ray : Ray, scene : &Scene, depth : i32) -> alg::Vector3<f32> {
	if let Some(hit) = scene.hit(&ray, 0.00001, 999.9) {
		if DEBUG_NORMALS {
			let normal_colour = 0.5 * alg::Vector3::new(hit.normal.x + 1.0,
			hit.normal.y + 1.0,
			hit.normal.z + 1.0);
			return normal_colour;
		}

		if depth < 100 {
			if let Some(bounce) = hit.material.bounce(&ray, 0.0001, 999.9, &hit) {
				return multiply_colour(bounce.attenuation, colour(bounce.ray, &scene, depth + 1)) * bounce.absorption;
			}
			else {
				return alg::Vector3::new(0.0, 0.0, 0.0);
			}
		}
		else {
			return alg::Vector3::new(1.0, 0.0, 1.0);
		}
	}

	let unit_dir = alg::normalize::<alg::Vector3<f32>>(&ray.direction());
	let t = 0.5 * (unit_dir.x + 1.0);
	(1.0 - t) * alg::Vector3::new(1.0, 1.0, 1.0) + t * alg::Vector3::new(1.0, 0.0, 0.0)
}

fn main()
{	
	let upscale = 1.0;
	let width = (1920 as f32 * upscale) as usize;
	let height = (1080 as f32 * upscale) as usize;
	let mut zoom = 0.5;
	let fov = width as f32 / 1280.0 * zoom;
	let aspect = width as f32 / height as f32;
	let mut lower_left_corner = alg::Vector3::new(-5.0 * zoom, -2.0 * zoom, -10.0);
	let horizontal = alg::Vector3::new(4.0 / upscale * aspect * fov, 0.0, 0.0);
	let vertical = alg::Vector3::new(0.0, 4.0 / upscale * fov, 0.0);
	let mut origin = alg::Vector3::new(0.0, 0.0, 0.0);

	let mut scene1 = Scene{hitables : Vec::new()};
	
	let sphere1 = Sphere::new(alg::Vector3::new(0.0, 0.0, -5.0), 0.5, alg::Vector3::new(1.0, 0.2, 0.2), Box::new(Diffuse::new()));
	scene1.hitables.push(Box::new(sphere1));
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(1.0, 0.0, -7.0), 2.2, alg::Vector3::new(1.0, 1.0, 0.0), Box::new(Diffuse::new()))));
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(1.0, 0.5, -5.0), 0.75, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Metal::new(0.0)))));
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(-1.0, 0.0, -4.0), 0.3, alg::Vector3::new(0.0, 1.0, 0.0), Box::new(Diffuse::new()))));
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(-0.5, 0.0, -4.0), 0.3, alg::Vector3::new(0.0, 0.3, 0.8), Box::new(Diffuse::new()))));
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(0.0, -19.2, -10.0), 20.0, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Metal::new(0.1)))));
	//scene.hitables.push(Box::new(Cube::new(alg::Vector3::new(2.0, 1.0, -2.7), 1.0, alg::Vector3::new(0.9, 1.0, 0.9), Box::new(Diffuse::new()))));
	//scene.hitables.push(Box::new(Cube::new(alg::Vector3::new(-2.0, 1.0, -2.7), 1.0, alg::Vector3::new(0.9, 1.0, 0.9), Box::new(Diffuse::new()))));
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(-0.75, 0.0, -2.0), 0.25, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Metal::new(0.0)))));
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(-1.05, 0.0, -2.0), 0.25, alg::Vector3::new(0.3, 0.3, 1.0), Box::new(Diffuse::new()))));
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(0.55, 0.0, -2.0), 0.25, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Metal::new(0.0)))));
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(0.75, 0.4, -2.0), 0.25, alg::Vector3::new(1.0, 0.3, 0.3), Box::new(Diffuse::new()))));
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(0.95, -0.25, -1.9), 0.25, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Diffuse::new()))));

	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(0.0, -100.0, 0.0), 50.0, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Diffuse::new()))));
	/*for i in 0..7 {
		scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(-1.5 + (i as f32 / 2.0), 0.5, -3.0), 0.25, alg::Vector3::new(0.3, 0.3, 1.0), Box::new(Diffuse::new()))));
		scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(-1.5 + (i as f32 / 2.0), 0.6, -6.0), 0.1, alg::Vector3::new(0.3, 1.0, 0.3), Box::new(Diffuse::new()))));
	}
	
	scene1.hitables.push(Box::new(Sphere::new(alg::Vector3::new(0.0, -19.2, -10.0), 20.0, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Metal::new(0.1)))));
	let mut vertices = [alg::Vector3::new(0.0, 0.0, 0.0); 3];
	vertices[2] = alg::Vector3::new(-0.5, 1.0, -7.0);
	vertices[1] = alg::Vector3::new(0.0, 1.5, -4.0);
	vertices[0] = alg::Vector3::new(0.5, 1.0, -3.0);
	let triangle = Triangle::new(vertices, alg::Vector3::new(0.0, 0.0, 0.0), alg::Vector3::new(1.0, 0.2, 0.2), Box::new(Diffuse::new()));
	scene1.hitables.push(Box::new(triangle));*/
	//scene1.hitables.push(Box::new(Cube::new(alg::Vector3::new(-0.95, -0.25, -1.9), 0.25, alg::Vector3::new(1.0, 1.0, 1.0), Box::new(Diffuse::new()))));

	let fox = tobj::load_obj(&Path::new("fox/Fox.obj"));
	assert!(fox.is_ok());
	let (models, materials) = fox.unwrap();
	print_model_info(&models, &materials);
	println!("# of models: {}", models.len());
	println!("# of materials: {}", materials.len());
	
	for (i, m) in models.iter().enumerate() {
		let mesh = &m.mesh;
		println!("model[{}].name = \'{}\'", i, m.name);
		println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

		println!("Size of model[{}].indices: {}", i, mesh.indices.len());
		/*for f in 0..mesh.indices.len() / 3 {
			println!("    idx[{}] = {}, {}, {}.", f, mesh.indices[3 * f],
				mesh.indices[3 * f + 1], mesh.indices[3 * f + 2]);
		}*/

		println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
		assert!(mesh.positions.len() % 3 == 0);
		/*for v in 0..mesh.positions.len() / 3 {
			println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
				mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);
		}*/

		println!("Triangles: {}", mesh.indices.len() / 3);
	}
	println!("hi");
	/*for (i, m) in materials.iter().enumerate() {
		println!("material[{}].name = \'{}\'", i, m.name);
		println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1],
			m.ambient[2]);
		println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1],
			m.diffuse[2]);
		println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1],
			m.specular[2]);
		println!("    material.Ns = {}", m.shininess);
		println!("    material.d = {}", m.dissolve);
		println!("    material.map_Ka = {}", m.ambient_texture);
		println!("    material.map_Kd = {}", m.diffuse_texture);
		println!("    material.map_Ks = {}", m.specular_texture);
		println!("    material.map_Ns = {}", m.normal_texture);
		println!("    material.map_d = {}", m.dissolve_texture);
		for (k, v) in &m.unknown_param {
			println!("    material.{} = {}", k, v);
		}
	}*/
	
	for (_i, m) in models.iter().enumerate() {
		let mesh = &m.mesh;
		assert!(mesh.indices.len() % 3 == 0);
		//Convert the indices that map to vertices in to a vector that stores the actual vertices values (one per index)
		let mut vertices : Vec<alg::Vector3<f32>> = vec![];
		let mut normals : Vec<alg::Vector3<f32>> = vec![];
		println!("hi2");
		for i in 0..mesh.indices.len() / 3 {
			vertices.push(alg::Vector3::new(
			mesh.positions[(3 * mesh.indices[3 * i]) as usize],
			mesh.positions[(3 * mesh.indices[3 * i] + 1) as usize],
			mesh.positions[(3 * mesh.indices[3 * i] + 2) as usize]));

			vertices.push(alg::Vector3::new(
			mesh.positions[(3 * mesh.indices[3 * i + 1]) as usize],
			mesh.positions[(3 * mesh.indices[3 * i + 1] + 1) as usize],
			mesh.positions[(3 * mesh.indices[3 * i + 1] + 2) as usize]));


			vertices.push(alg::Vector3::new(
			mesh.positions[(3 * mesh.indices[3 * i + 2]) as usize],
			mesh.positions[(3 * mesh.indices[3 * i + 2] + 1) as usize],
			mesh.positions[(3 * mesh.indices[3 * i + 2] + 2) as usize]));


			if mesh.normals.len() != 0
			{
				normals.push(alg::Vector3::new(
				mesh.normals[(3 * mesh.indices[3 * i]) as usize],
				mesh.normals[(3 * mesh.indices[3 * i] + 1) as usize],
				mesh.normals[(3 * mesh.indices[3 * i] + 2) as usize]));

				normals.push(alg::Vector3::new(
				mesh.normals[(3 * mesh.indices[3 * i + 1]) as usize],
				mesh.normals[(3 * mesh.indices[3 * i + 1] + 1) as usize],
				mesh.normals[(3 * mesh.indices[3 * i + 1] + 2) as usize]));
				
				normals.push(alg::Vector3::new(
				mesh.normals[(3 * mesh.indices[3 * i + 2]) as usize],
				mesh.normals[(3 * mesh.indices[3 * i + 2] + 1) as usize],
				mesh.normals[(3 * mesh.indices[3 * i + 2] + 2) as usize]));
			}
		}

		println!("hi2");

		/*println!("Vertices: {}", vertices.len());

		for v in 0..vertices.len(){
			println!("v[{}] = ({}, {}, {})",
			v, vertices[v].x, vertices[v].y, vertices[v].z);
		}*/

		//for every 3 real vertices create a triangle
		let scale = 0.2;
		let offset = alg::Vector3::new(-1.0, 1.0, -4.0);
		assert!(vertices.len() % 3 == 0);
		for v in 0..vertices.len() / 3 {
			let mut verts = [alg::Vector3::new(0.0, 0.0, 0.0); 3];
			verts[0] = vertices[3 * v];
			verts[1] = vertices[3 * v + 1];
			verts[2] = vertices[3 * v + 2];

			verts[0] *= scale;
			verts[1] *= scale;
			verts[2] *= scale;

			verts[0] += offset;
			verts[1] += offset;
			verts[2] += offset;

			let mut normal = alg::Vector3::new(0.0, 0.0, 0.0);
			if mesh.normals.len() != 0
			{
				normal = normals[3 * v];
			}

			let triangle = Triangle::new(verts, normal, alg::Vector3::new(0.0, 1.0, 1.0), Box::new(Diffuse::new()));
			//scene1.hitables.push(Box::new(triangle));
		}
	}

	let mut scene = Scene{hitables : Vec::new()};
	if USE_BVH {
		scene.hitables.push(Box::new(BVHNode::new(scene1.hitables, 0.0, 999.9)));
	}
	else {
		scene = scene1;
	}

	let mut samples = 512;
	if DEBUG_NORMALS {
		samples = 1;
	}

	let mut lights : Vec<Light> = Vec::new();

	if USE_LIGHTS {
		lights.push(Light::new(alg::Vector3::<f32>::new(0.0, 0.0, 0.0), 10.0, alg::Vector3::<f32>::new(1.0, 1.0, 1.0)))
	}

	for frames in 0..1 {
		let raw_colours: Vec<Vec<alg::Vector3<f32>>> = (0..height).into_par_iter().map(|y| {
			let row: Vec<alg::Vector3<f32>> = (0..width).into_par_iter().map(|x| {
				let mut rng = rand::thread_rng();
				let mut col  = alg::Vector3::new(0.0, 0.0, 0.0);
				for _s in 0..samples {
					let rand_u : f32 = rng.gen();
					let rand_v : f32 = rng.gen();

					let u : f32 = (x as f32 + rand_u) / width as f32;
					let v : f32 = (y as f32 + rand_v) / height as f32;

					let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
					let temp_c : alg::Vector3<f32> = colour(ray, &scene, 0);
					
					col.x = col.x + temp_c.x;
					col.y = col.y + temp_c.y;
					col.z = col.z + temp_c.z;
				}
				col = col / samples as f32;
				col
			}).collect();
			if y % ((height as f32 / 100.0) as usize) == 0 {
				let percent = y as f32 / height as f32;
				println!("{}%", (percent * 100.0) as i32);
			}
			row
		}).collect();

		let mut pixels : Vec<u8> = Vec::with_capacity(width * height * 3);

		println!("Done raycasting frame{}", frames);

		for y in (0..height).rev() {
			for x in 0..width {
				let mut col = raw_colours[y][x];
				col = alg::Vector3::new((col.x).sqrt(), (col.y).sqrt(), (col.z).sqrt());
				let ir = (255.99 * col.x) as u8;
				let ig = (255.99 * col.y) as u8;
				let ib = (255.99 * col.z) as u8;
				pixels.push(ir);
				pixels.push(ig);
				pixels.push(ib);
			}
		}

		println!("Saving png frame{}", frames);
		let filename = format!("frame{}.png", frames);
		let path = Path::new(&filename);
		let display = path.display();

		println!("Saving png2 frame{}", frames);
		let file = match File::create(&path) {
			Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
			Ok(file) => file,
		};

		println!("Saving png3 frame{}", frames);
		let writer = BufWriter::new(&file);
		let out_image = image::png::PNGEncoder::new(writer);
		let _okay = match out_image.encode(&pixels, width as u32, height as u32, image::ColorType::RGB(8)) {
			Err(why) => panic!("hmm? {}", why.description()),
			Ok(okay) => okay,
		};

		//per frame scene changes
		origin.x += 0.1;
		origin.z += 0.1;
		println!("Saving png4 frame{}", frames);
	}

	println!("Done");
}