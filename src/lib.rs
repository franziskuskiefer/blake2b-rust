
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

fn compress(h: &mut[u64; 8], m: [u64; 16], t: [u64; 2], f: bool) {
    let mut v: [u64; 16] = [0; 16];

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

// TODO: make input flexible and u8
// TODO: add key
pub fn blake2b(data: [u64; 16]) -> [u8; 64] {
    let f: bool = true; // XXX: This takes only one block at the moment. So it's the last.

    let mut t: [u64; 2] = [0; 2];
    t = inc_counter(t, 3);

    let mut h: [u64; 8] = [0; 8];
    for i in 0..8 {
      h[i] = IV[i];
    }
    h[0] = h[0] ^ 0x01010000 ^ 64; // This only supports len = 64

    compress(&mut h, data, t, f);

    // Make h little endian u8 array and return.
    make_u8array(&h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let mut m: [u64; 16] = [0; 16];
        m[0] = 0x0000000000636261u64;

        let h = blake2b(m);

        let expected = [
            0xba, 0x80, 0xa5, 0x3f, 0x98, 0x1c, 0x4d, 0x0d, 0x6a, 0x27, 0x97,
            0xb6, 0x9f, 0x12, 0xf6, 0xe9, 0x4c, 0x21, 0x2f, 0x14, 0x68, 0x5a,
            0xc4, 0xb7, 0x4b, 0x12, 0xbb, 0x6f, 0xdb, 0xff, 0xa2, 0xd1, 0x7d,
            0x87, 0xc5, 0x39, 0x2a, 0xab, 0x79, 0x2d, 0xc2, 0x52, 0xd5, 0xde,
            0x45, 0x33, 0xcc, 0x95, 0x18, 0xd3, 0x8a, 0xa8, 0xdb, 0xf1, 0x92,
            0x5a, 0xb9, 0x23, 0x86, 0xed, 0xd4, 0x00, 0x99, 0x23
        ];
        assert_eq!(&expected[..], &h[..]);
    }
}
