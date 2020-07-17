use std::fmt;

#[derive(Debug)]
struct Image {
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Off = 0,
    On = 1,
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pixel::On => f.write_str("#"),
            Pixel::Off => f.write_str(" "),
        }
    }
}

#[derive(Clone, Debug)]
struct Glyph {
    pixels: Vec<Pixel>,
    width: usize,
}

impl std::convert::From<std::vec::Vec<std::vec::Vec<Pixel>>> for Glyph {
    fn from(pixels: Vec<Vec<Pixel>>) -> Self {
        let mut collected_pixels: Vec<Pixel> = vec![];
        let height = pixels.first().unwrap().len();

        for y in 0..height {
            for column in &pixels {
                collected_pixels.push(*column.get(y).unwrap());
            }
        }

        Self {
            pixels: collected_pixels,
            width: pixels.len(),
        }
    }
}

#[derive(Debug)]
enum Symbol {
    Num(u8),
    Ellipsis,
    Unknown(Glyph),
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

#[allow(dead_code)]
fn print(Image { pixels }: &Image) {
    print_pixels(pixels);
}

#[allow(dead_code)]
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

fn decode(image: Image) -> Vec<Vec<Symbol>> {
    return image
        .pixels
        .split(|row| is_blank(row))
        .filter(|group| group.len() > 0)
        .map(|group| parse_line(group.to_vec()))
        .collect();
}

fn parse_line(pixels: Vec<Vec<Pixel>>) -> Vec<Symbol> {
    let width = pixels.first().unwrap().len();
    let height = pixels.len();
    let mut glyphs: Vec<Glyph> = vec![];
    let mut current_group: Vec<Vec<Pixel>> = vec![];
    let mut saw_blank = false;

    for i in 0..width {
        let column: Vec<Pixel> = pixels.iter().map(|row| *row.get(i).unwrap()).collect();

        if column.iter().all(|pixel| *pixel == Pixel::Off) {
            if saw_blank && current_group.len() > 0 {
                glyphs.push(current_group.into());
                current_group = vec![];
            }
            saw_blank = true;
        } else {
            if saw_blank && current_group.len() > 0 {
                current_group.push(vec![Pixel::Off; height]);
            }
            current_group.push(column);
            saw_blank = false;
        }
    }

    if current_group.len() > 0 {
        glyphs.push(current_group.into());
    }

    return glyphs.iter().map(|glyph| decode_glyph(glyph)).collect();
}

fn decode_glyph(glyph: &Glyph) -> Symbol {
    // need to trim bottom of "stick number" glyphs
    match glyph.width {
        1 => match glyph.pixels.as_slice() {
            [Pixel::On, Pixel::On] => Symbol::Num(1),
            _ => Symbol::Unknown(glyph.clone()),
        },
        2 => match glyph.pixels.as_slice() {
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::Off] => Symbol::Num(0),
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On] => Symbol::Num(1),
            _ => Symbol::Unknown(glyph.clone()),
        },
        3 => match glyph.pixels.as_slice() {
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::On, Pixel::On, Pixel::Off, Pixel::Off] => {
                Symbol::Num(2)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::Off] => {
                Symbol::Num(3)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::Off, Pixel::On, Pixel::On, Pixel::Off] => {
                Symbol::Num(4)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::On, Pixel::On, Pixel::Off] => {
                Symbol::Num(5)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::Off] => {
                Symbol::Num(6)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::On, Pixel::Off] => {
                Symbol::Num(7)
            }
            [Pixel::Off, Pixel::On, Pixel::On, Pixel::On, Pixel::Off, Pixel::Off, Pixel::On, Pixel::Off, Pixel::On] => {
                Symbol::Num(8)
            }
            _ => Symbol::Unknown(glyph.clone()),
        },
        4 => match glyph.pixels.as_slice() {
            [Pixel::On, Pixel::On, Pixel::On, Pixel::On] => Symbol::Ellipsis,
            _ => Symbol::Unknown(glyph.clone()),
        },
        _ => Symbol::Unknown(glyph.clone()),
    }
}

fn main() {
    let image = parse("tests/files/message1.png");
    //    print(&image);

    println!("{:?}", decode(image));
}

// #[cfg(test)]
// fn message1() {
//     assert_eq!(
//         decode(parse("tests/files/message1.png")),
//         [
//             vec![Symbol::Num(0)],
//             vec![Symbol::Num(1), Symbol::Num(1)],
//             vec![Symbol::Num(2), Symbol::Num(2)],
//             vec![Symbol::Num(3), Symbol::Num(3)],
//             vec![Symbol::Num(4), Symbol::Num(4)],
//             vec![Symbol::Num(5), Symbol::Num(5)],
//             vec![Symbol::Num(6), Symbol::Num(6)],
//             vec![Symbol::Num(7), Symbol::Num(7)],
//             vec![Symbol::Num(8), Symbol::Num(8)],
//             vec![Symbol::Ellipsis]
//         ]
//     )
// }
