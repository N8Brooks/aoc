use std::array;

#[derive(Debug, Clone)]
pub struct Md5Hasher {
    state: [u32; 4],
    buffer: [u8; 64],
    len: u64,
}

impl Default for Md5Hasher {
    fn default() -> Self {
        Self {
            state: Self::INIT_STATE,
            buffer: [0; 64],
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
            self.buffer[buf_len..buf_len + bytes.len()].copy_from_slice(bytes);
            return;
        };
        self.buffer[buf_len..].copy_from_slice(left);
        Self::process(&mut self.state, &self.buffer);
        let (chunks, rem) = right.as_chunks::<64>();
        for chunk in chunks {
            Self::process(&mut self.state, chunk);
        }
        self.buffer[..rem.len()].copy_from_slice(rem);
    }

    #[inline]
    pub fn write_u8(&mut self, i: u8) {
        let buf_len = self.buf_len();
        self.buffer[buf_len] = i;
        if buf_len == 63 {
            Self::process(&mut self.state, &self.buffer);
        }
        self.len = self.len.wrapping_add(1);
    }

    #[inline]
    fn buf_len(&self) -> usize {
        (self.len & 63) as usize
    }

    pub fn finish(&self) -> [u32; 4] {
        let mut hasher = self.clone();
        let bit_len = hasher.len.wrapping_mul(8).to_le_bytes();
        hasher.write_u8(0x80);
        let buf_len = hasher.buf_len();
        if buf_len > 56 {
            hasher.buffer[buf_len..].fill(0);
            Self::process(&mut hasher.state, &hasher.buffer);
            hasher.buffer[..56].fill(0);
        } else {
            hasher.buffer[buf_len..56].fill(0);
        }
        hasher.buffer[56..64].copy_from_slice(&bit_len);
        Self::process(&mut hasher.state, &hasher.buffer);
        hasher.state
    }

    pub fn finish_bytes(&self) -> [u8; 16] {
        let digest = self.finish();
        let mut words = [0; 16];
        for (word, bytes) in words.as_chunks_mut::<4>().0.iter_mut().zip(digest) {
            word.copy_from_slice(&bytes.to_le_bytes());
        }
        words
    }

    pub fn finish_hex(&self) -> [u8; 32] {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let digest = self.finish_bytes();
        let mut out = [0; 32];
        for (i, byte) in digest.into_iter().enumerate() {
            out[2 * i] = HEX[(byte >> 4) as usize];
            out[2 * i + 1] = HEX[(byte & 0x0f) as usize];
        }
        out
    }

    pub fn finish_u128(&self) -> u128 {
        u128::from_be_bytes(self.finish_bytes())
    }

    fn process(state: &mut [u32; 4], buffer: &[u8; 64]) {
        const S: [u32; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20,
            5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
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

        let words: [u32; 16] = array::from_fn(|i| {
            let j = 4 * i;
            u32::from_le_bytes([buffer[j], buffer[j + 1], buffer[j + 2], buffer[j + 3]])
        });

        let [mut a, mut b, mut c, mut d] = *state;

        for i in 0..16 {
            let f = (b & c) | (!b & d);
            let t = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(words[i])
                .rotate_left(S[i]);
            (a, b, c, d) = (d, b.wrapping_add(t), b, c);
        }

        for i in 16..32 {
            let f = (d & b) | (!d & c);
            let g = (5 * i + 1) & 15;
            let t = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(words[g])
                .rotate_left(S[i]);
            (a, b, c, d) = (d, b.wrapping_add(t), b, c);
        }

        for i in 32..48 {
            let f = b ^ c ^ d;
            let g = (3 * i + 5) & 15;
            let t = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(words[g])
                .rotate_left(S[i]);
            (a, b, c, d) = (d, b.wrapping_add(t), b, c);
        }

        for i in 48..64 {
            let f = c ^ (b | !d);
            let g = (7 * i) & 15;
            let t = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(words[g])
                .rotate_left(S[i]);
            (a, b, c, d) = (d, b.wrapping_add(t), b, c);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_md5() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(&hasher.finish_hex(), b"9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_finish_u128() {
        let mut hasher = Md5Hasher::default();
        hasher.write(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(hasher.finish_u128(), 0x9e107d9d372bb6826bd81d3542a419d6,);
    }
}
