use std::io::{self, Read};

#[derive(Debug)]
struct Layer {
    pixels: std::vec::Vec<u32>,
}

impl Layer {
    pub fn from_str(buf: &str) -> Self {
        Self {
            pixels: buf.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        }
    }

    pub fn count_digit(&self, d: u32) -> i32 {
        self.pixels.iter().filter(|&&pixel| pixel == d).count() as i32
    }
}

#[derive(Debug)]
struct Image {
    width: u32,
    height: u32,
    layers: std::vec::Vec<Layer>,
}

impl Image {
    pub fn from_str(buf: &str, width: u32, height: u32) -> Self {
        let pixels_per_frame = width * height;
        let total_frames = buf.len() as u32 / pixels_per_frame;
        let mut layers = vec![];

        for i in 0..total_frames {
            let frame_start = (i * pixels_per_frame) as usize;
            let frame_end = frame_start + pixels_per_frame as usize;
            layers.push(Layer::from_str(&buf[frame_start..frame_end]));
        }

        Self {
            width,
            height,
            layers,
        }
    }

    pub fn get_layer_with_fewer_zeros(&self) -> &Layer {
        let mut layer_with_fewer_zeros = &self.layers[0];
        let mut min_zeros = layer_with_fewer_zeros.count_digit(0);

        for i in 1..self.layers.len() {
            let layer = &self.layers[i];
            let zeros = layer.count_digit(0);
            if min_zeros == -1 || zeros < min_zeros {
                min_zeros = zeros;
                layer_with_fewer_zeros = layer;
            }
        }

        layer_with_fewer_zeros
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let image = Image::from_str(&buf.trim(), 25, 6);
    let layer_with_fewer_zeros = image.get_layer_with_fewer_zeros();
    let ones = layer_with_fewer_zeros.count_digit(1);
    let twos = layer_with_fewer_zeros.count_digit(2);
    println!("{:?}", ones * twos);

    Ok(())
}
