use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma};
use std::cmp::max;

pub fn calculate_edginess(
    center: [u8; 4],
    left: [u8; 4],
    right: [u8; 4],
    top: [u8; 4],
    bottom: [u8; 4],
) -> u16 {
    let d_l = helper(center, left);
    let d_r = helper(center, right);
    let d_t = helper(center, top);
    let d_b = helper(center, bottom);
    max(max_helper(d_l, d_r), max_helper(d_t, d_b))
}

pub fn max_helper(d1: [u8; 4], d2: [u8; 4]) -> u16 {
    max(
        d1.iter().map(|&a| a as u16).sum::<u16>(),
        d2.iter().map(|&a| a as u16).sum::<u16>(),
    )
}

pub fn helper(center: [u8; 4], other: [u8; 4]) -> [u8; 4] {
    let calculate_rgba_diff =
        |idx1, idx2| -> u8 { (center[idx1] as i16 - other[idx2] as i16).unsigned_abs() as u8 };
    [
        calculate_rgba_diff(0, 0),
        calculate_rgba_diff(1, 1),
        calculate_rgba_diff(2, 2),
        calculate_rgba_diff(3, 3),
    ]
}

pub fn edge_detection(img: &DynamicImage, width: u32, height: u32) -> GrayImage {
    let mut edginess: GrayImage = ImageBuffer::new(width - 1, height - 1);
    // println!("INSIDE");
    for iy in 1..height - 1 {
        for ix in 1..width - 1 {
            let center = img.get_pixel(ix, iy);
            let left = img.get_pixel(ix - 1, iy);
            let right = img.get_pixel(ix + 1, iy);
            let top = img.get_pixel(ix, iy + 1);
            let bottom = img.get_pixel(ix, iy - 1);
            let lol1_1 = calculate_edginess(center.0, left.0, right.0, top.0, bottom.0);
            let lol = Luma([lol1_1 as u8]);
            edginess.put_pixel(ix - 1, iy - 1, lol);
        }
    }
    edginess
}
