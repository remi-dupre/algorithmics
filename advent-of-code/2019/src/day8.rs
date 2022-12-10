use std::iter;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part_1(input: &str) -> usize {
    let layer = input
        .as_bytes()
        .chunks(WIDTH * HEIGHT)
        .min_by_key(|layer| bytecount::count(layer, b'0'))
        .expect("empty image");

    bytecount::count(layer, b'1') * bytecount::count(layer, b'2')
}

pub fn part_2(input: &str) -> String {
    let mut buffer = [None; WIDTH * HEIGHT];

    for layer in input.as_bytes().chunks(WIDTH * HEIGHT) {
        for (buf, lay) in buffer.iter_mut().zip(layer.iter()) {
            *buf = buf.or_else(|| match lay {
                b'0' => Some(false),
                b'1' => Some(true),
                b'2' => None,
                _ => panic!("invalid pixel value"),
            })
        }
    }

    buffer
        .chunks(WIDTH)
        .flat_map(|line| {
            line.iter()
                .map(|pix| match pix {
                    None => panic!("undefined pixel"),
                    Some(true) => '#',
                    Some(false) => ' ',
                })
                .chain(iter::once('\n'))
        })
        .collect()
}
