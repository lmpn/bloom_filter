use bit_vec::BitVec;
use rand::Rng;
use siphasher::sip::SipHasher24;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    filter: BitVec,
    hashes: Vec<SipHasher24>,
}

impl BloomFilter {
    fn new(m: usize, k: usize) -> BloomFilter {
        let mut rng = rand::thread_rng();
        let hashes = std::ops::Range { start: 0, end: k }
            .into_iter()
            .map(|_| SipHasher24::new_with_keys(rng.gen(), rng.gen()))
            .collect::<Vec<SipHasher24>>();
        BloomFilter {
            filter: BitVec::from_elem(m as usize, false),
            hashes,
        }
    }

    pub fn new_with_bits(m: usize, n: usize) -> BloomFilter {
        let k = ((m / n) as f64 * 2.0f64.log10()).ceil();
        Self::new(m, k as usize)
    }
    pub fn new_with_hashes(k: usize, n: usize) -> BloomFilter {
        let m = ((k * n) as f64 / 2.0f64.log10()).ceil();
        Self::new(m as usize, k)
    }

    fn calculate_hashes<T: Hash>(&self, x: &T) -> Vec<usize> {
        self.hashes
            .iter()
            .map(|it| {
                let mut cit = it.clone();
                x.hash(&mut cit);
                cit.finish() as usize % self.filter.len()
            })
            .collect()
    }

    pub fn set<T: Hash>(&mut self, x: &T) {
        self.calculate_hashes(x)
            .iter()
            .for_each(|index| self.filter.set(*index, true));
    }

    pub fn test<T: Hash>(&self, x: &T) -> bool {
        self.calculate_hashes(x)
            .iter()
            .map(|index| self.filter.get(*index))
            .map(|opt| opt.unwrap_or(false))
            .all(|it| it)
    }

    pub fn test_and_set<T: Hash>(&mut self, x: &T) -> bool {
        if self.test(x) {
            return true;
        }
        self.set(x);
        false
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use crate::BloomFilter;

    #[test]
    fn test_and_insert() {
        let mut bf = BloomFilter::new_with_bits(100, 10);
        let mut bf2 = BloomFilter::new_with_hashes(3, 10);
        let value = 10;
        let value2 = "string";
        let value3 = vec![1, 2, 3];
        bf.set(&value);
        bf.set(&value2);
        bf2.set(&value);
        bf2.set(&value2);
        assert_eq!(bf.test(&value), bf2.test(&value));
        assert_eq!(bf.test(&value2), bf2.test(&value2));
        assert_eq!(bf.test(&value2), bf2.test(&value2));
        assert_eq!(bf.test(&value3), bf2.test(&value3));
        assert_eq!(bf.test(&value3), false);
        assert_eq!(bf2.test(&value3), false);
    }

    #[test]
    fn test_and_set_one_shot() {
        let mut bf = BloomFilter::new_with_bits(100, 10);
        let value = 10;
        let value2 = "string";
        let value3 = vec![1, 2, 3];

        assert_eq!(bf.test_and_set(&value), false);
        assert_eq!(bf.test_and_set(&value2), false);
        assert_eq!(bf.test_and_set(&value3), false);
        assert_eq!(bf.test_and_set(&value), true);
        assert_eq!(bf.test_and_set(&value2), true);
        assert_eq!(bf.test_and_set(&value3), true);
    }
}
