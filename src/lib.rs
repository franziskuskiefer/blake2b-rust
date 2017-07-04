
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
    (v >> l)|(v << (64 - l))
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

#[allow(unused_variables)] // f is not used yet
fn compress(h: &mut[u64; 8], m: [u64; 16], t: [u64; 2], f: [u64; 2]) {
    let mut v: [u64; 16] = [0; 16];

    // Prepare.
    for i in 0..8 {
        v[i] = h[i];
    }
    for i in 8..16 {
        v[i] = IV[i-8];
    }
    v[12] ^= t[0];
    v[13] ^= t[1];

    // TODO: check last block flag f.

    // Mixing.
    for i in 0..12 {
        for j in 0..16 {
          println!("{},{}: {:x}", i, j, v[j]);
        }
        println!("{:?}", SIGMA[i]);
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

    println!("{:?}", h);
}

// TODO: make input flexible and u8
// TODO: add key
pub fn blake2b(data: [u64; 16]) -> [u64; 8] {
    let f: [u64; 2] = [0; 2];
    let t: [u64; 2] = [0; 2];

    let mut h: [u64; 8] = [0; 8];
    for i in 0..8 {
      h[i] = IV[i];
    }
    h[0] = h[0] ^ 0x01010000 ^ 64; // This only support len = 64
    println!("{:?}", h);

    compress(&mut h, data, t, f);
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let mut m: [u64; 16] = [0; 16];
        m[0] = 0x0000000000636261u64;
        print!("m: ");
        for j in 0..16 {
          print!("{:016x} ", m[j]);
        }
        println!("");

        let h = blake2b(m);

        print!("h: ");
        for j in 0..8 {
          print!("{:x}", h[j]);
        }
        println!("");
    }
}
