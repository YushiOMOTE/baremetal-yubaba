#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: usize,
}

impl Pixel {
    fn new(x: usize, y: usize, color: usize) -> Self {
        Self { x, y, color }
    }
}

pub struct FontWriter {
    blocksize: usize,
    buf: BytesMut,
}

impl FontWriter {
    pub fn new(blocksize: usize) -> Self {
        let mut buf = BytesMut::new();

        buf.put_u64(blocksize as u64);

        Self { blocksize, buf }
    }

    pub fn add(&mut self, width: usize, bitmap: &[u8]) {
        self.buf.put_u64(width as u64);
        self.buf.put_u64(bitmap.len() as u64);
        self.buf.extend_from_slice(bitmap);
        let padlen = self.buf.len() + self.blocksize - bitmap.len();
        self.buf.resize(padlen, 0);
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf.to_vec()
    }
}

pub struct FontReader<'a> {
    blocksize: usize,
    buf: &'a [u8],
}

impl<'a> FontReader<'a> {
    pub fn new(mut buf: &'a [u8]) -> Self {
        let blocksize = buf.get_u64() as usize;
        Self { blocksize, buf }
    }

    pub fn get(&self, ch: char) -> (usize, usize, Vec<Pixel>) {
        let unit = self.blocksize + 16;
        let mut buf = &self.buf[unit * ch as usize..];
        let width = buf.get_u64() as usize;
        let size = buf.get_u64() as usize;
        let height = if width > 0 { size / width } else { 0 };
        let bitmap = &buf[..size];

        (
            width,
            height,
            (0..size)
                .map(|p| {
                    let x = p % width;
                    let y = p / width;
                    let color = bitmap[p] as usize;
                    Pixel::new(x, y, color)
                })
                .collect(),
        )
    }
}

#[test]
fn readwrite() {
    use alloc::vec;

    let mut wr = FontWriter::new(10);

    wr.add(2, &[1, 2, 3, 4, 5, 6]);
    wr.add(3, &[4, 5, 6, 7, 8, 9, 10, 11, 12]);
    wr.add(1, &[13, 14]);

    let buf = wr.into_inner();

    let rd = FontReader::new(&buf);

    fn p(x: usize, y: usize, c: usize) -> Pixel {
        Pixel::new(x, y, c)
    }

    assert_eq!(rd.blocksize, 10);
    assert_eq!(
        rd.get(0 as char).2,
        vec![
            p(0, 0, 1),
            p(1, 0, 2),
            p(0, 1, 3),
            p(1, 1, 4),
            p(0, 2, 5),
            p(1, 2, 6)
        ]
    );
    assert_eq!(
        rd.get(1 as char).2,
        vec![
            p(0, 0, 4),
            p(1, 0, 5),
            p(2, 0, 6),
            p(0, 1, 7),
            p(1, 1, 8),
            p(2, 1, 9),
            p(0, 2, 10),
            p(1, 2, 11),
            p(2, 2, 12),
        ]
    );
    assert_eq!(rd.get(2 as char).2, vec![p(0, 0, 13), p(0, 1, 14),]);
}
