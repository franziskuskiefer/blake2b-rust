
#[allow(dead_code)]
static SIGMA: [[usize; 16]; 12] = [
   [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ],
   [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3 ],
   [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4 ],
   [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8 ],
   [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13 ],
   [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9 ],
   [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11 ],
   [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10 ],
   [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5 ],
   [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0 ],
   [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ],
   [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3 ]
];

#[allow(dead_code)]
static IV: [u64; 8] = [
    0x6a09e667f3bcc908u64, 0xbb67ae8584caa73bu64, 0x3c6ef372fe94f82bu64,
    0xa54ff53a5f1d36f1u64, 0x510e527fade682d1u64, 0x9b05688c2b3e6c1fu64,
    0x1f83d9abfb41bd6bu64, 0x5be0cd19137e2179u64
];

fn rotr(v: u64, l: u8) -> u64 {
    (v >> l) ^ (v << (64 - l))
}

fn mix(v: &mut[u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = rotr(v[d] ^ v[a], 32);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = rotr(v[b] ^ v[c], 24);

    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = rotr(v[d] ^ v[a], 16);

    v[c] = v[c].wrapping_add(v[d]);
    v[b] = rotr(v[b] ^ v[c], 63);
}

fn inc_counter(t: [u64; 2], x: u64) -> [u64; 2] {
  let mut result: [u64; 2] = [0; 2];
  result[0] = t[0].wrapping_add(x);
  if result[0] < x {
    result[1] = t[1] + 1;
  }
  result
}

fn make_u8array(h: &[u64; 8]) -> [u8; 64] {
    let mut result: [u8; 64] = [0; 64];
    for i in (0..8).rev() {
        result[0+(i*8)] = (h[i] & 0xFFu64) as u8;
        result[1+(i*8)] = ((h[i] & 0xFF00u64) >> 8) as u8;
        result[2+(i*8)] = ((h[i] & 0xFF0000u64) >> 16) as u8;
        result[3+(i*8)] = ((h[i] & 0xFF000000u64) >> 24) as u8;
        result[4+(i*8)] = ((h[i] & 0xFF00000000u64) >> 32) as u8;
        result[5+(i*8)] = ((h[i] & 0xFF0000000000u64) >> 40) as u8;
        result[6+(i*8)] = ((h[i] & 0xFF000000000000u64) >> 48) as u8;
        result[7+(i*8)] = ((h[i] & 0xFF00000000000000u64) >> 56) as u8;
    }
    result
}

fn make_u64array(h: &[u8; 128]) -> [u64; 16] {
    let mut result: [u64; 16] = [0; 16];
    for i in 0..16 {
        result[i] = h[0+8*i] as u64 | (h[1+8*i] as u64) << 8 |
                    (h[2+8*i] as u64) << 16 | (h[3+8*i] as u64) << 24 |
                    (h[4+8*i] as u64) << 32 | (h[5+8*i] as u64) << 40 |
                    (h[6+8*i] as u64) << 48 | (h[7+8*i] as u64) << 56;
    }
    result
}

#[allow(dead_code)]
fn print_block(x: &[u8; 128]) {
    for i in 0..128 {
        print!("{:?}, ", x[i]);
    }
    println!("");
}

fn compress(h: &mut[u64; 8], m: [u8; 128], t: [u64; 2], f: bool) {
    let mut v: [u64; 16] = [0; 16];

    // Read u8 data to u64.
    let m = make_u64array(&m);

    // Prepare.
    for i in 0..8 {
        v[i] = h[i];
        v[i+8] = IV[i];
    }
    v[12] = v[12] ^ t[0];
    v[13] = v[13] ^ t[1];
    if f {
      v[14] = !v[14];
    }

    // Mixing.
    for i in 0..12 {
        mix(&mut v, 0, 4,  8, 12, m[SIGMA[i][ 0]], m[SIGMA[i][ 1]]);
        mix(&mut v, 1, 5,  9, 13, m[SIGMA[i][ 2]], m[SIGMA[i][ 3]]);
        mix(&mut v, 2, 6, 10, 14, m[SIGMA[i][ 4]], m[SIGMA[i][ 5]]);
        mix(&mut v, 3, 7, 11, 15, m[SIGMA[i][ 6]], m[SIGMA[i][ 7]]);
        mix(&mut v, 0, 5, 10, 15, m[SIGMA[i][ 8]], m[SIGMA[i][ 9]]);
        mix(&mut v, 1, 6, 11, 12, m[SIGMA[i][10]], m[SIGMA[i][11]]);
        mix(&mut v, 2, 7,  8, 13, m[SIGMA[i][12]], m[SIGMA[i][13]]);
        mix(&mut v, 3, 4,  9, 14, m[SIGMA[i][14]], m[SIGMA[i][15]]);
    }

    for i in 0..8 {
        h[i] = h[i] ^ v[i] ^ v[i + 8];
    }
}

// TODO: add key
pub fn blake2b(data: Vec<u8>) -> [u8; 64] {
    let mut last_block: bool = false;
    let mut t: [u64; 2] = [0; 2];

    let mut h: [u64; 8] = [0; 8];
    for i in 0..8 {
      h[i] = IV[i];
    }
    h[0] = h[0] ^ 0x01010000 ^ 64; // This only supports the 512 version.

    let blocks = (data.len() / 128) as usize;
    for i in 0..blocks {
        let mut m: [u8; 128] = [0; 128];
        m.clone_from_slice(&data[0+i*128 .. 0+i*128+128]);
        t = inc_counter(t, 128u64);
        compress(&mut h, m, t, last_block);
    }

    last_block = true;
    // Pad last bits of data to a full block.
    let mut m: [u8; 128] = [0; 128];
    let remaining_bytes = data.len() - 128 * blocks;
    let remaining_start = data.len() - remaining_bytes;
    t = inc_counter(t, remaining_bytes as u64);
    let mut j = 0;
    for i in remaining_start..(remaining_start + remaining_bytes) {
        m[j] = data[i];
        j += 1;
    }
    compress(&mut h, m, t, last_block);

    // Make h little endian u8 array and return.
    make_u8array(&h)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXPECTED_ABC: [u8; 64] = [
        0xba, 0x80, 0xa5, 0x3f, 0x98, 0x1c, 0x4d, 0x0d, 0x6a, 0x27, 0x97,
        0xb6, 0x9f, 0x12, 0xf6, 0xe9, 0x4c, 0x21, 0x2f, 0x14, 0x68, 0x5a,
        0xc4, 0xb7, 0x4b, 0x12, 0xbb, 0x6f, 0xdb, 0xff, 0xa2, 0xd1, 0x7d,
        0x87, 0xc5, 0x39, 0x2a, 0xab, 0x79, 0x2d, 0xc2, 0x52, 0xd5, 0xde,
        0x45, 0x33, 0xcc, 0x95, 0x18, 0xd3, 0x8a, 0xa8, 0xdb, 0xf1, 0x92,
        0x5a, 0xb9, 0x23, 0x86, 0xed, 0xd4, 0x00, 0x99, 0x23
    ];

    #[test]
    fn test_single_block() {
        let m: Vec<u8> = vec![0x61u8, 0x62, 0x63];
        let h = blake2b(m);
        assert_eq!(&EXPECTED_ABC[..], &h[..]);
    }

    #[test]
    fn test_single_block_string() {
        let m = String::from("abc");
        let h = blake2b(m.into_bytes());
        assert_eq!(&EXPECTED_ABC[..], &h[..]);
    }

    #[test]
    fn test_multi_block_string() {
        let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
        let h = blake2b(m.into_bytes());

        let expected: [u8; 64] = [
            0x5c, 0xc9, 0x7c, 0x7f, 0x9f, 0xf2, 0x00, 0x8b, 0x40, 0x12, 0x6f,
            0x37, 0x3f, 0x43, 0x33, 0xfa, 0x34, 0x8d, 0x9f, 0x50, 0x06, 0xb8,
            0x73, 0x57, 0xa6, 0xd8, 0x61, 0x12, 0xa1, 0xa0, 0x43, 0x95, 0x4d,
            0xa2, 0xe2, 0x8f, 0x01, 0xb2, 0xf9, 0x55, 0xa9, 0x32, 0xfb, 0x8a,
            0x8d, 0x0a, 0x17, 0x87, 0xd0, 0xc6, 0xd9, 0x62, 0x77, 0x9c, 0xbc,
            0x58, 0xbf, 0xdf, 0x89, 0x48, 0x1c, 0x87, 0x46, 0x96
        ];
        assert_eq!(&expected[..], &h[..]);
    }

    #[test]
    fn test_multi_block_string_longer() {
        let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
        let h = blake2b(m.into_bytes());

        let expected: [u8; 64] = [
            0x1f, 0x9e, 0xe6, 0x5a, 0xa0, 0x36, 0x05, 0xfc, 0x41, 0x0e, 0x2f, 0x55,
            0x96, 0xfd, 0xb5, 0x9d, 0x85, 0x95, 0x5e, 0x24, 0x37, 0xe7, 0x0d, 0xe4,
            0xa0, 0x22, 0x4a, 0xe1, 0x59, 0x1f, 0x97, 0x03, 0x57, 0x54, 0xf0, 0xca,
            0x92, 0x75, 0x2f, 0x9e, 0x86, 0xeb, 0x82, 0x4f, 0x9c, 0xf4, 0x02, 0x17,
            0x7f, 0x76, 0x56, 0x26, 0x46, 0xf4, 0x07, 0xfd, 0x1f, 0x78, 0xdb, 0x7b,
            0x0d, 0x24, 0x43, 0xf0
        ];
        assert_eq!(&expected[..], &h[..]);
    }
}
