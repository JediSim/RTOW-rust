use image;
use glam::f64::DVec3;
use crate::ray::Ray;

pub type Color = DVec3;

fn hit_sphere(center: DVec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let b = -2.0 * oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - (4.0 * a * c) as f64;
    return discriminant >= 0.0;
    
}

pub fn write_color(image: &mut image::RgbImage, i: u32, j: u32, color: Color) {
    let r = (255.999 * color[0]) as u8;
    let g = (255.999 * color[1]) as u8;
    let b = (255.999 * color[2]) as u8;
    image.put_pixel(i, j, image::Rgb([r, g, b]));
}

pub fn ray_color(ray: &Ray) -> Color {
    // put sphere in the scene
    if hit_sphere(DVec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return DVec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction;
    let a = 0.5 * (unit_direction[1] + 1.0);
    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
}
