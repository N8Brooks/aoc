use std::{array, mem};

pub fn part_1(input: &str) -> usize {
    pad_key(input, 0)
}

pub fn part_2(input: &str) -> usize {
    pad_key(input, 2016)
}

fn pad_key(input: &str, k: usize) -> usize {
    let mut counts = [0usize; 16];
    let mut it = (0..)
        .map({
            let prefix = input.as_bytes();
            let mut num_buf = [0u8; 20];
            let mut block = [0u8; 64];
            let mut m = [0u32; 16];
            block[..prefix.len()].copy_from_slice(prefix);
            move |i| {
                let mut digest = md5(prefix, Some(i), &mut num_buf, &mut block, &mut m);
                for _ in 0..k {
                    let tmp = digest.map(|b| match b {
                        0..=9 => b'0' + b,
                        10..=15 => b'a' + (b - 10),
                        _ => panic!("invalid hex digit {b}"),
                    });
                    digest = md5(&tmp, None, &mut num_buf, &mut block, &mut m);
                }
                digest
            }
        })
        .map(move |digest| {
            let three = digest
                .array_windows()
                .find(|[a, b, c]| a == b && b == c)
                .map(|[a, ..]| usize::from(*a));
            digest
                .array_windows()
                .filter(|[a, b, c, d, e]| a == b && b == c && c == d && d == e)
                .for_each(|[a, ..]| counts[*a as usize] += 1);
            (three, counts)
        });
    let mut window: [_; 1000] = it.next_chunk().unwrap();
    (0..)
        .filter(move |i| {
            let pair @ (_, right) = it.next().unwrap();
            let (three, left) = mem::replace(&mut window[i % 1000], pair);
            three.is_some_and(|j| right[j] > left[j])
        })
        .nth(63)
        .unwrap()
}

fn md5(
    prefix: &[u8],
    num: Option<usize>,
    num_buf: &mut [u8; 20],
    block: &mut [u8; 64],
    m: &mut [u32; 16],
) -> [u8; 32] {
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
    const SEEDS: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
    let digits = num.map_or([].as_slice(), |num| write_decimal(num, num_buf));
    let msg_len = prefix.len() + digits.len();
    debug_assert!(msg_len <= 55, "input too long for single-chunk MD5");

    block[..56].fill(0);
    block[..prefix.len()].copy_from_slice(prefix);
    block[prefix.len()..msg_len].copy_from_slice(digits);
    block[msg_len] = 0x80;
    let bit_len = (msg_len as u64) * 8;
    block[56..64].copy_from_slice(&bit_len.to_le_bytes());

    for (word, bytes) in m.iter_mut().zip(block.as_chunks().0) {
        *word = u32::from_le_bytes(*bytes);
    }

    let [mut a, mut b, mut c, mut d] = SEEDS;
    for (i, (s, k)) in S.into_iter().zip(K).enumerate() {
        let (f, g) = if i < 16 {
            ((b & c) | (!b & d), i)
        } else if i < 32 {
            ((d & b) | (!d & c), (5 * i + 1) % 16)
        } else if i < 48 {
            (b ^ c ^ d, (3 * i + 5) % 16)
        } else {
            (c ^ (b | !d), (7 * i) % 16)
        };
        (a, b, c, d) = (
            d,
            b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(k)
                    .wrapping_add(m[g])
                    .rotate_left(s),
            ),
            b,
            c,
        );
    }
    let words = [
        a.wrapping_add(SEEDS[0]).to_le(),
        b.wrapping_add(SEEDS[1]).to_le(),
        c.wrapping_add(SEEDS[2]).to_le(),
        d.wrapping_add(SEEDS[3]).to_le(),
    ];
    array::from_fn(|i| {
        let word = words[i / 8];
        let nib = (i % 8) ^ 1;
        ((word >> (4 * nib)) & 0xF) as u8
    })
}

fn write_decimal(mut num: usize, buf: &mut [u8; 20]) -> &[u8] {
    if num == 0 {
        buf[19] = b'0';
        return &buf[19..20];
    }
    let mut idx = 20;
    while num > 0 {
        idx -= 1;
        buf[idx] = (num % 10) as u8 + b'0';
        num /= 10;
    }
    &buf[idx..20]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE: &str = "abc";

    #[test_case(EXAMPLE => 22728)]
    #[test_case(INPUT => 18626)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 20092)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
