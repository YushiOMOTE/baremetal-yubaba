use std::convert::TryInto;

use fonts::*;

fn main() {
    let font = include_bytes!("./font.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let max = (0u32..0x9faf)
        .map(|v| font.rasterize(v.try_into().unwrap(), 10.0).1.len())
        .max();

    let mut wr = FontWriter::new(max.unwrap());

    (0u32..0x9faf).for_each(|v| {
        let (metrics, bitmap) = font.rasterize(v.try_into().unwrap(), 10.0);
        wr.add(metrics.width, &bitmap);
    });

    std::fs::write("../src/font.bin", wr.into_inner()).unwrap();
}
