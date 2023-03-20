use image::{DynamicImage, GenericImageView};
// use lab::Lab;
use rayon::iter::*;

pub(crate) fn mse(img1: &DynamicImage, img2: &DynamicImage) -> f32 {
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
