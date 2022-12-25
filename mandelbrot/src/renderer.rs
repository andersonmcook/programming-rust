use num::Complex;

pub fn parallel_render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let rows_per_band = bounds.1 / num_cpus::get() + 1;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    crossbeam::scope(|spawner| {
        bands.into_iter().enumerate().for_each(|(i, band)| {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| render(band, band_bounds, band_upper_left, band_lower_right));
        })
    })
    .unwrap()
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let height = upper_left.im - lower_right.im;
    let width = lower_right.re - upper_left.re;

    Complex {
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        Complex {
            im: -0.75,
            re: -0.5
        },
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { im: 1.0, re: -1.0 },
            Complex { im: -1.0, re: 1.0 }
        )
    )
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { im: 0.0, re: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
