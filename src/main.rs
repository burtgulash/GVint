use std::mem;

extern crate rand;

struct GVInt {
    size: usize,
    headers: Vec<u8>,
    xs: Vec<u8>,
    buf: Vec<u32>,
}

fn byte_size2(mut x: usize) -> usize {
    if x == 0 {
        return 1;
    }

    let mut log = 0;
    while x > 0 {
        log += 1;
        x /= 256;
    }
    return log;
}

fn byte_size(x: usize) -> usize {
    if x > 0xFFFFFF {
        4
    } else if x > 0xFFFF {
        3
    } else if x > 0xFF {
        2
    } else {
        1
    }
}

impl GVInt {
    fn new() -> GVInt {
        GVInt {
            size: 0,
            xs: Vec::new(),
            headers: Vec::new(),
            buf: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn byte_size(&self) -> usize {
        mem::size_of::<usize>()
            + self.xs.len()
            + self.headers.len()
            + self.buf.len() * mem::size_of::<u32>()
    }

    fn add(&mut self, x: u32) {
        if self.buf.len() == 4 {
            let mut header = 0;
            for (i, x) in self.buf.drain(..).enumerate() {
                let size = byte_size(x as usize);
                header |= (size - 1) << (6 - i * 2);
                match size {
                    1 => {
                        self.xs.push((x >> 0) as u8);
                    },
                    2 => {
                        self.xs.push((x >> 8) as u8);
                        self.xs.push((x >> 0) as u8);
                    },
                    3 => {
                        self.xs.push((x >> 16) as u8);
                        self.xs.push((x >> 8) as u8);
                        self.xs.push((x >> 0) as u8);
                    },
                    4 => {
                        self.xs.push((x >> 24) as u8);
                        self.xs.push((x >> 16) as u8);
                        self.xs.push((x >> 8) as u8);
                        self.xs.push((x >> 0) as u8);
                    }
                    _ => {},
                }
            }
            self.headers.push(header as u8);
        }

        self.buf.push(x);
        self.size += 1;
    }

    fn _decode_one(&self, xs: &[u8], offset: usize, size: usize) -> u32 {
        unsafe {
            match size {
                4 => {
                      (*xs.get_unchecked(offset + 0) as u32) << 24
                    | (*xs.get_unchecked(offset + 1) as u32) << 16
                    | (*xs.get_unchecked(offset + 2) as u32) << 8
                    | (*xs.get_unchecked(offset + 3) as u32) << 0
                },
                3 => {
                      (*xs.get_unchecked(offset + 0) as u32) << 16
                    | (*xs.get_unchecked(offset + 1) as u32) << 8
                    | (*xs.get_unchecked(offset + 2) as u32) << 0
                },
                2 => {
                      (*xs.get_unchecked(offset + 0) as u32) << 8
                    | (*xs.get_unchecked(offset + 1) as u32) << 0
                },
                1 => {
                      (*xs.get_unchecked(offset + 0) as u32) << 0
                },
                _ => 0,
            }
        }
    }

    fn decode(&self) -> Vec<u32> {
        let mut r = Vec::new();
        let mut offset = 0;

        macro_rules! decode_one {
            ( $header:expr, $pos:expr ) => {
                let size = (3 & ($header >> $pos)) as usize + 1;
                r.push(self._decode_one(&self.xs, offset, size));
                offset += size;
            };
        }

        for &h in &self.headers {
            decode_one!(h, 6);
            decode_one!(h, 4);
            decode_one!(h, 2);
            decode_one!(h, 0);
        }

        for &x in &self.buf {
            r.push(x);
        }

        r
    }
}

fn main() {
    let mut input = Vec::new();
    let mut compressed = GVInt::new();

    for _ in 0 .. 10000000 {
        let x = rand::random::<u16>();
        input.push(x as u32);
    }

    for &x in &input {
        compressed.add(x);
    }

    let decompressed = compressed.decode();

    //println!("input: {:?}", input);
    //println!("decompressed: {:?}", decompressed);
    //println!("compressed: {:?}", compressed.xs);

    debug_assert_eq!(input, decompressed);

    println!("Byte size: {}, u32 size: {}, deco size: {}", compressed.byte_size(), compressed.size() * 4, decompressed.len());
}
