use image;
use glam::f64::DVec3;
use crate::ray::Ray;

pub type Color = DVec3;

pub fn format_color(color: Color) -> String {
    format!("{} {} {}", (255.999 * color[0]) as u64,
                        (255.999 * color[1]) as u64,
                        (255.999 * color[2]) as u64)
}

fn normalize(v: DVec3) -> DVec3 {
    let length = v.length();
    return v / length;
}

fn hit_sphere(center: DVec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let h = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h*h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-h - discriminant.sqrt()) / a;
    }
    
}

pub fn write_color(image: &mut image::RgbImage, i: u32, j: u32, color: Color) {
    let r = (255.999 * color[0]) as u8;
    let g = (255.999 * color[1]) as u8;
    let b = (255.999 * color[2]) as u8;
    image.put_pixel(i, j, image::Rgb([r, g, b]));
}

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(DVec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - DVec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.50 * DVec3::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0);
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction[1] + 1.0);
    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
}
