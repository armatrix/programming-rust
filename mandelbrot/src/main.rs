use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;
use std::u32;

// fn complex_square_add_loop(c: Complex<f64>) {
//     let mut x = Complex { re: 0.0, im: 0.0 };
//     loop {
//         x = x * x + c
//     }
// }

// Option的定义，这里可以稍微关注下不同语言对错误的处理方式，像try...catch...finally 和 if err!= nil
// enum Option<T> {
//     None,
//     Some(T),
// }
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut x = Complex { re: 0.0, im: 0.0 };

    // 这里和大多数语言一样的，也是左开右闭
    for i in 0..limit {
        if x.norm_sqr() > 4.0 {
            return Some(i);
        }
        x = x * x + c
    }
    None
}

// 对trait的使用介绍对入参的限制，泛型函数的定义和使用
// 返回值是元组类型，类似python中的元组
// 函数的声明类似C++中的模版函数
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

// 当处理成功的时候并没有任何有效的值返回，注意rust和其他语言在main函数的不同，很多语言的main函数通过返回不同的整型数值来代表不同的状态，通常1是成功
// 在rust中，其返回的不是1或者非1的状态
fn gen_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    // 这里是对一种常见范式的简写，通常我们会遇到处理y=f(x）时，成功返回y，或者失败返回错误的信息，注意类似这种写法不能在main函数中，因为rust中main函数不返回值
    // 需要使用match语句或者一些类似unwrap或者expect的语句，这些在后面补充
    // let output = match File::open(filename) {
    //     Ok(file) => file,
    //     Err(err) => return Err(err),
    // };
    let output = File::create(&filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // sigle thread
    // render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                // 闭包 注意这里的move
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    gen_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

// cmd
// cargo run mandel.png 400x300 -1.20,0.35 -1,0.20
// type -a time
// time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
