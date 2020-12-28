extern crate image;

struct CustomPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl CustomPixel {
    fn to_u8_array(&self) -> [u8;4] {
        return [self.r, self.g, self.b, self.a];
    }
}

pub fn pixelate_image(path: &str, output: &str, block_count: usize) {
    let image = image::open(path).unwrap().into_rgba8();
    let width = image.width() as usize;
    let height = image.height() as usize;
    if !verify_block_count(width, height, block_count) {
        println!("Invalid block count");
        std::process::exit(1);
    }
    let xblocks: usize = width / block_count;
    let yblocks: usize = width / block_count;
    let total_blocks = xblocks * yblocks;
    let mut rgba_averages: Vec<CustomPixel> = Vec::with_capacity(total_blocks);
    for x in (0..width).step_by(xblocks) {
        for y in (0..height).step_by(yblocks) {
            let mut counter: u32 = 0;
            let mut sumr: u32 = 0;
            let mut sumg: u32 = 0;
            let mut sumb: u32 = 0;
            let mut suma: u32 = 0;
            for xcount in x..x+xblocks {
                for ycount in y..y+yblocks {
                    let pixel = image.get_pixel(xcount as u32, ycount as u32);
                    sumr += pixel[0] as u32;
                    sumg += pixel[1] as u32;
                    sumb += pixel[2] as u32;
                    suma += pixel[3] as u32;
                    counter += 1;
                }
            }
            sumr /= counter;
            sumg /= counter;
            sumb /= counter;
            suma /= counter;
            let average: CustomPixel = CustomPixel{ r: sumr as u8, g: sumg as u8, b: sumb as u8, a: suma as u8 };
            rgba_averages.push(average);
        }
    }

    let mut pixelated_image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(width as u32, height as u32);
    let mut counter: usize = 0;
    for x in (0..width).step_by(xblocks) {
        for y in (0..height).step_by(yblocks) {
            let current_pixel: &CustomPixel = &rgba_averages[counter];
            for xcount in x..x+xblocks {
                for ycount in y..y+yblocks {
                    pixelated_image_buffer.put_pixel(xcount as u32, ycount as u32, image::Rgba(current_pixel.to_u8_array()));
                }
            }
            counter += 1;
        }
    }
    pixelated_image_buffer.save(output).unwrap();
}

fn verify_block_count(width: usize, height: usize, block_count: usize) -> bool {
    return width % block_count == 0 && height % block_count == 0;
}

pub fn find_possible_block_counts(width: usize, height: usize) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    for counter in 1..width {
        if width % counter == 0 && height % counter == 0 {
            values.push(counter);
        }
    }
    values
}