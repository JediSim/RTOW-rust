use image::{ImageBuffer, RgbImage, Rgb};  

const IMAGE_WIDTH:u32 = 256;
const IMAGE_HEIGHT:u32 = 256;

fn main() {

    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // u32 for next calculation
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            // convert in u8 for r and g
            image.put_pixel(i,j, Rgb::<u8>([i.try_into().unwrap(), j.try_into().unwrap(), 0]));
        }
    }

    image.save("output.png").unwrap();
    println!("Hello, world!");
}
