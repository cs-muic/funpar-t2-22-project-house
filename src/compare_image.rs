use clap::builder::TypedValueParser;
use image::{DynamicImage, GenericImageView, Rgba};
use rand::Rng;
// use lab::Lab;
use rayon::iter::*;

pub fn best_shapes(
    img1: &DynamicImage,
    shape_stats: &(u32, u32),
    colors: &Vec<[u8; 4]>,
    canvas: &DynamicImage,
) -> (Option<(Vec<(u32, u32)>, [u8; 4])>) {
    use deltae::*;
    use lab::Lab;

    let (img_width, img_height) = (img1.width(), img1.height());
    // println!("got image pixels");
    // Create image size
    let mut rng = rand::thread_rng();
    // let shape_size = rng.gen_range(1..((img1.height() + img1.width())/2));
    let shape_size = rng.gen_range(1..200);
    let shape_placement_coords = (shape_stats.0, shape_stats.1);
    // println!("generated image size");
    // Produce the coords of every pixel in the shape
    let mut shape_coords = Vec::new();
    for y in 0..shape_size {
        let mut current_row = Vec::new();
        for x in 0..shape_size {
            current_row.push((shape_placement_coords.0 + x, shape_placement_coords.1 + y));
        }
        shape_coords.push(current_row);
    }
    // println!("produced every pixel");
    // Filter out coords that are out of bounds
    let shape_coords_2 = shape_coords.into_par_iter().flatten();
    let shape_coords_3 = shape_coords_2
        .into_par_iter()
        .filter(|a| (a.0 < img_width && a.1 < img_height))
        .collect::<Vec<(u32, u32)>>();

    #[derive(Clone, Copy)]
    struct MyLab(f32, f32, f32);

    // Types that implement Into<LabValue> also implement the Delta trait
    impl From<MyLab> for LabValue {
        fn from(mylab: MyLab) -> Self {
            LabValue {
                l: mylab.0,
                a: mylab.1,
                b: mylab.2,
            }
        }
    }
    // Implement DeltaEq for your own types
    impl<D: Delta + Copy> DeltaEq<D> for MyLab {}

    let mut rng = rand::thread_rng();
    let current_rgb = colors[rng.gen_range(0..colors.len())];

    let compare_new = shape_coords_3.clone().into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let pic2 = Lab::from_rgba(&current_rgb);
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    let compare_canvas = shape_coords_3.clone().into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let pic2 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    let new_deltaE: f32 = compare_new.into_par_iter().map(|a| *a.value()).sum();
    let canvas_deltaE: f32 = compare_canvas.into_par_iter().map(|a| *a.value()).sum();
    if new_deltaE <= canvas_deltaE {
        return Some((shape_coords_3, current_rgb));
    }
    return None;
    // println!("whattttt{}",ans);
}

// pub fn get_next_best(size_coef: f32, imf: &DynamicImage, colors: &Vec<[u8; 4]>) -> (u32, u32, u32, u32, [u8; 4]) {
//
// }

pub fn produce_next_best_shape(
    coords: &Vec<(u32, u32)>,
    img: &DynamicImage,
    colors: &Vec<[u8; 4]>,
    canvas: &DynamicImage,
) -> Vec<Option<(Vec<(u32, u32)>, [u8; 4])>> {
    // TODO: Take in the output image and compare the cost of that versus the cost of the new random instead of generating so many squares.
    let output = coords
        .chunks(1024)
        .into_iter()
        .flat_map(|chunk| {
            let output = chunk
                .par_iter()
                .map(|coord| best_shapes(img, coord, colors, canvas))
                .collect::<Vec<Option<(Vec<(u32, u32)>, [u8; 4])>>>();
            output
        })
        .collect();
    // let output = coords
    //     .into_par_iter()
    //     .map(|coord| {
    //         best_shapes(img, coord, colors, canvas)
    //             // .collect::<Vec<Option<(Vec<(u32, u32)>, [u8; 4])>>>();
    //     }).collect();

    // let output = coords
    //     .chunks(1024)
    //     .into_iter()
    //     .map(|coords| {
    //         coords
    //             .into_par_iter()
    //             .map(|coord| best_shapes(img, coord, colors, canvas))
    //             .collect()
    //     })
    //     .collect();
    return output;
}

pub fn compare(img1: &DynamicImage, img2: &DynamicImage) -> f32 {
    use deltae::*;
    use lab::Lab;

    // let (width, height) = img1.dimensions();
    // let mut sum = 0.0;
    let imag1 = img1.pixels();
    let imag2 = img2.pixels();
    // let imag1 = Lab::from_rgb(&[253, 120, 138]);
    // let imag2 = Lab::from_rgb()

    #[derive(Clone, Copy)]
    struct MyLab(f32, f32, f32);

    // Types that implement Into<LabValue> also implement the Delta trait
    impl From<MyLab> for LabValue {
        fn from(mylab: MyLab) -> Self {
            LabValue {
                l: mylab.0,
                a: mylab.1,
                b: mylab.2,
            }
        }
    }
    // Implement DeltaEq for your own types
    impl<D: Delta + Copy> DeltaEq<D> for MyLab {}
    let a: _ = imag1.zip(imag2).collect::<Vec<_>>();

    // let diff = a.iter().flat_map(|(a, b)| a.iter().zip(b.iter()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let compare = a.into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(&a.0 .2 .0);
        let pic2 = Lab::from_rgba(&a.1 .2 .0);
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    compare.into_par_iter().map(|a| *a.value()).sum()
    // println!("whattttt{}",ans);
}
