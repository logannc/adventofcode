use crate::utils::errors::Error;
use crate::utils::files::problem_input_path;

use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pixel {
    Black,
    White,
    Transparent,
    Unknown(u32),
}

impl From<u32> for Pixel {
    fn from(num: u32) -> Self {
        match num {
            0 => Self::Black,
            1 => Self::White,
            2 => Self::Transparent,
            n => Self::Unknown(n),
        }
    }
}

impl Pixel {
    fn mix(&self, covered_pixel: Pixel) -> Pixel {
        match (self, covered_pixel) {
            (Pixel::Black, _) => Pixel::Black,
            (Pixel::White, _) => Pixel::White,
            (_, other) => other,
        }
    }
    fn raster(&self) -> char {
        match self {
            Pixel::Black => ' ',
            Pixel::White => '*',
            Pixel::Transparent => '_',
            _ => 'x',
        }
    }
}

fn to_layers<T>(mut v: Vec<T>, layer_size: usize) -> Vec<Vec<T>> {
    let mut layers = VecDeque::new();
    while v.len() > layer_size {
        layers.push_front(v.split_off(v.len() - layer_size));
    }
    layers.push_front(v);
    layers.into()
}

pub fn part_one() -> Result<u32, Error> {
    let input_path = problem_input_path(8, None);
    let img: Vec<u32> = fs::read_to_string(&input_path)?
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let image_width = 25;
    let image_height = 6;
    let layer_size = image_height * image_width;
    let layers = to_layers(img, layer_size);
    if let Some(min_layer) = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|&&pixel| pixel == 0).count())
    {
        let mut ones = 0;
        let mut twos = 0;
        for &pixel in min_layer {
            if pixel == 1 {
                ones += 1;
            } else if pixel == 2 {
                twos += 1;
            }
        }
        Ok(ones * twos)
    } else {
        Err(Error::NoSolutionFound)
    }
}

pub fn part_two() -> Result<u32, Error> {
    let input_path = problem_input_path(8, None);
    let pixels: Vec<Pixel> = fs::read_to_string(&input_path)?
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.into())
        .collect();
    let image_width = 25;
    let image_height = 6;
    let layer_size = image_height * image_width;
    let layers = to_layers(pixels, layer_size);
    let mut rasterized_pixels = Vec::new();
    for idx in 0..layer_size {
        let mut pixel = Pixel::Transparent;
        for layer in layers.iter() {
            pixel = pixel.mix(layer[idx]);
            if pixel != Pixel::Transparent {
                break;
            }
        }
        rasterized_pixels.push(pixel)
    }
    for (pixel, i) in rasterized_pixels.into_iter().zip(0..layer_size) {
        if i % image_width == 0 {
            println!("");
        }
        print!("{}", pixel.raster());
    }
    println!("");
    Ok(0)
}
