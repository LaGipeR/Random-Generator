use long_int::LongInt;

pub struct RandGen {
    x: u32,
}

impl RandGen {
    // const A: u32 = 134775813u32;
    // const C: u32 = 1u32;
    const A: u32 = 214013u32;
    const C: u32 = 2531011u32;
    pub fn new(seed: u32) -> RandGen {
        RandGen { x: seed }
    }

    pub fn new_from_time() -> RandGen {
        let mut res = Self::new(0);
        res.set_seed_from_time();
        res
    }

    pub fn set_seed_from_time(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let seed = ((seed >> 64) ^ seed) as u64;
        let seed = ((seed >> 32) ^ seed) as u32;

        self.set_seed(seed);
    }

    pub fn set_seed(&mut self, new_seed: u32) {
        self.x = new_seed;
    }

    pub fn next(&mut self, l: u32, r: u32) -> u32 {
        assert!(l <= r);

        let next = (Self::A as u64) * (self.x as u64) + (Self::C as u64);

        // self.x = ((next >> 32) as u32) ^ ((next & (u32::MAX as u64)) as u32);
        // self.x = (next >> 32) as u32;
        self.x = next as u32;

        return if l == 0 && r == u32::MAX {
            self.x
        } else {
            self.x % (r - l + 1) + l
        };
    }

    const FIRST_16_BITS: u32 = 0b1111111111111111;
    pub fn next_long_int(&mut self, l: &LongInt, r: &LongInt) -> LongInt {
        assert!(&l <= &r);

        let mut result = LongInt::new();

        while &result <= &r {
            result = (&result) << 16;

            let rand_value = self.next(0, u32::MAX);
            result = result
                + LongInt::from_blocks_big_endian(vec![
                    (rand_value >> 16) ^ (rand_value & Self::FIRST_16_BITS),
                ]);
        }

        result = (result % (r - l + LongInt::from_blocks_big_endian(vec![1]))) + (l);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_rand_u32() {
        let l = 1234;
        let r = 1234567;

        let mut gen = RandGen::new(0);
        gen.set_seed_from_time();

        let mut s = HashSet::new();
        const N: usize = 100;
        for _ in 0..N {
            let val = gen.next(l, r);

            println!("{val}");
            s.insert(val);

            assert!(l <= val && val <= r);
        }

        println!("{}", s.len());
    }

    #[test]
    fn test_rand_long() {
        let l = LongInt::new();
        let r = (LongInt::from_blocks_big_endian(vec![3]) << 400)
            - LongInt::from_blocks_big_endian(vec![1]);

        let mut gen = RandGen::new(0);
        gen.set_seed_from_time();

        const N: usize = 100;
        for _ in 0..N {
            let val = gen.next_long_int(&l, &r);

            println!("{}", val.getHex());

            assert!(l <= val && val <= r);
        }
    }
}
