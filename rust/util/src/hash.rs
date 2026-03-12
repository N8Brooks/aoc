use std::mem::{self, MaybeUninit};

#[derive(Debug, Clone)]
pub struct Md5Hasher {
    state: [u32; 4],
    buffer: [u32; 16],
    len: u64,
}

impl Default for Md5Hasher {
    fn default() -> Self {
        Self {
            state: Self::INIT_STATE,
            #[allow(clippy::uninit_assumed_init, invalid_value)]
            // SAFETY: The buffer is initialized before it's read from.
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
}

impl Md5Hasher {
    const INIT_STATE: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

    #[inline]
    pub fn write(&mut self, bytes: &[u8]) {
        let buf_len = self.buf_len();
        self.len = self.len.wrapping_add(bytes.len() as u64);
        let Some((left, right)) = bytes.split_at_checked(64 - buf_len) else {
            self.buf()[buf_len..buf_len + bytes.len()].copy_from_slice(bytes);
            return;
        };
        self.buf()[buf_len..].copy_from_slice(left);
        self.process();
        let (chunks, rem) = right.as_chunks::<64>();
        for chunk in chunks {
            // SAFETY: process converts the u32s into little-endian
            let tmp = unsafe { *chunk.as_ptr().cast() };
            self.state = process(&self.state, tmp);
        }
        self.buf()[..rem.len()].copy_from_slice(rem);
    }

    #[inline]
    pub fn write_u8(&mut self, i: u8) {
        let buf_len = self.buf_len();
        self.buf()[buf_len] = i;
        if buf_len == 63 {
            self.process();
        }
        self.len = self.len.wrapping_add(1);
    }

    #[inline(always)]
    fn buf(&mut self) -> &mut [u8; 64] {
        // SAFETY: idk
        unsafe { &mut *self.buffer.as_mut_ptr().cast() }
    }

    #[inline(always)]
    fn buf_len(&self) -> usize {
        (self.len & 63) as usize
    }

    pub fn finish(&self) -> [u8; 16] {
        #[cfg(target_endian = "big")]
        // SAFETY: idk
        unsafe {
            mem::transmute(self.finish_words().map(u32::to_le))
        }
        // SAFETY: idk
        #[cfg(target_endian = "little")]
        unsafe {
            mem::transmute(self.finish_words())
        }
    }

    pub fn finish_words(&self) -> [u32; 4] {
        let mut hasher = self.clone();
        let bit_len = hasher.len.wrapping_mul(8).to_le_bytes();
        hasher.write_u8(0x80);
        let buf_len = hasher.buf_len();
        if buf_len > 56 {
            hasher.buf()[buf_len..].fill(0);
            hasher.process();
            hasher.buf()[..56].fill(0);
        } else {
            hasher.buf()[buf_len..56].fill(0);
        }
        hasher.buf()[56..64].copy_from_slice(&bit_len);
        hasher.process();
        hasher.state
    }

    pub fn finish_hex(&self) -> [u8; 32] {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let digest = self.finish();
        let mut out = [MaybeUninit::uninit(); 32];
        for (i, byte) in digest.into_iter().enumerate() {
            out[2 * i].write(HEX[(byte >> 4) as usize]);
            out[2 * i + 1].write(HEX[(byte & 0x0f) as usize]);
        }
        // SAFETY: the array has been fully initialized
        unsafe { MaybeUninit::array_assume_init(out) }
    }

    pub fn finish_u128(&self) -> u128 {
        #[cfg(target_endian = "big")]
        // SAFETY: idk
        unsafe {
            mem::transmute(self.finish_words().map(u32::to_le))
        }
        // SAFETY: idk
        #[cfg(target_endian = "little")]
        unsafe {
            mem::transmute(self.finish_words())
        }
    }

    fn process(&mut self) {
        self.state = process(&self.state, &self.buffer);
    }
}

fn process(state @ &[mut a, mut b, mut c, mut d]: &[u32; 4], buffer: &[u32; 16]) -> [u32; 4] {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    #[cfg(target_endian = "big")]
    buffer.iter_mut().for_each(|word| *word = word.to_le());

    for i in 0..16 {
        let f = (b & c) | (!b & d);
        let t = a
            .wrapping_add(f)
            .wrapping_add(K[i])
            .wrapping_add(buffer[i])
            .rotate_left(S[i]);
        (a, b, c, d) = (d, b.wrapping_add(t), b, c);
    }

    for i in 16..32 {
        let f = (d & b) | (!d & c);
        let g = (5 * i + 1) & 15;
        let t = a
            .wrapping_add(f)
            .wrapping_add(K[i])
            .wrapping_add(buffer[g])
            .rotate_left(S[i]);
        (a, b, c, d) = (d, b.wrapping_add(t), b, c);
    }

    for i in 32..48 {
        let f = b ^ c ^ d;
        let g = (3 * i + 5) & 15;
        let t = a
            .wrapping_add(f)
            .wrapping_add(K[i])
            .wrapping_add(buffer[g])
            .rotate_left(S[i]);
        (a, b, c, d) = (d, b.wrapping_add(t), b, c);
    }

    for i in 48..64 {
        let f = c ^ (b | !d);
        let g = (7 * i) & 15;
        let t = a
            .wrapping_add(f)
            .wrapping_add(K[i])
            .wrapping_add(buffer[g])
            .rotate_left(S[i]);
        (a, b, c, d) = (d, b.wrapping_add(t), b, c);
    }

    [
        state[0].wrapping_add(a),
        state[1].wrapping_add(b),
        state[2].wrapping_add(c),
        state[3].wrapping_add(d),
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finish_hex() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(&hasher.finish_hex(), b"9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_finish_words() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            hasher.finish_words(),
            [0x9d7d109e, 0x82b62b37, 0x351dd86b, 0xd619a442]
        );
    }

    #[test]
    fn test_finish_bytes() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            &hasher.finish(),
            b"\x9e\x10\x7d\x9d\x37\x2b\xb6\x82\x6b\xd8\x1d\x35\x42\xa4\x19\xd6"
        );
    }

    #[test]
    fn test_finish_u128() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(hasher.finish_u128(), 0xd619a442351dd86b82b62b379d7d109e);
    }
}
