use std::fmt;

#[derive(Debug)]
struct Image {
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Clone, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pixel::On => f.write_str("#"),
            Pixel::Off => f.write_str(" "),
        }
    }
}

fn parse(path: &str) -> Image {
    let image = image::open(path).unwrap().to_luma();
    let width = image.dimensions().0 as usize;
    let height = image.dimensions().1 as usize;
    let scale = 4usize;
    let mut pixels = Vec::with_capacity(height / scale);

    for rows in image.into_vec().chunks(width * scale) {
        let row = &rows[0..width];
        let mut row_pixels = Vec::with_capacity(width / scale);

        for group in row.chunks(scale) {
            let pixel = group.first().unwrap();

            match pixel {
                0 => row_pixels.push(Pixel::Off),
                255 => row_pixels.push(Pixel::On),
                _ => panic!("wasn't expecting that!"),
            }
        }
        pixels.push(row_pixels);
    }

    remove_border(&mut pixels);

    return Image { pixels };
}

fn remove_border(pixels: &mut Vec<Vec<Pixel>>) {
    // remove top and bottom borders
    pixels.remove(0);
    pixels.remove(0);
    pixels.pop();
    pixels.pop();

    for row in pixels.iter_mut() {
        row.remove(0);
        row.remove(0);
        row.pop();
        row.pop();
    }
}

fn print(Image { pixels }: &Image) {
    print_pixels(pixels);
}

fn print_pixels(pixels: &Vec<Vec<Pixel>>) {
    for row in pixels {
        for pixel in row {
            match pixel {
                Pixel::Off => print!(" "),
                Pixel::On => print!("#"),
            }
        }
        println!();
    }
}

fn is_blank(pixels: &Vec<Pixel>) -> bool {
    pixels.iter().all(|pixel| *pixel == Pixel::Off)
}

// fn decode(pixels: Vec<Vec<Pixel>>) -> Vec<Vec<Symbol>>

fn main() {
    let image = parse("tests/files/message1.png");
    //    print(&image);

    for group in image.pixels.split(|row| is_blank(row)) {
        print_pixels(&group.to_vec());
        println!("---")
    }
}

#[cfg(test)]
fn message1() {
    //    assert_eq!(
    //        decode("tests/files/message1.png"),
    //        vec![
    //            vec![Num(0)],
    //            vec![Num(1), Num(1)],
    //            vec![Num(2), Num(2)],
    //            vec![Num(3), Num(3)],
    //            vec![Num(4), Num(4)],
    //            vec![Num(5), Num(5)],
    //            vec![Num(6), Num(6)],
    //            vec![Num(7), Num(7)],
    //            vec![Num(8), Num(8)],
    //            vec![Ellipsis]
    //        ]
    //    )
}
