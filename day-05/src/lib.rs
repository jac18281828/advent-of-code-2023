pub struct RangeMap {
    source_start: usize,
    destination_start: usize,
    range_width: usize,
}

impl RangeMap {
    pub fn new(source_start: usize, destination_start: usize, range_width: usize) -> Self {
        Self {
            source_start,
            destination_start,
            range_width,
        }
    }

    pub fn map(&self, source: usize) -> usize {
        if source < self.source_start || source >= self.source_start + self.range_width {
            source
        } else {
            source - self.source_start + self.destination_start
        }
    }

    pub fn is_in_range(&self, source: usize) -> bool {
        source >= self.source_start && source < self.source_start + self.range_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_map() {
        let range_map = RangeMap::new(0, 0, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);

        let range_map = RangeMap::new(0, 10, 10);
        assert_eq!(range_map.map(0), 10);
        assert_eq!(range_map.map(1), 11);
        assert_eq!(range_map.map(9), 19);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);

        let range_map = RangeMap::new(10, 0, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 0);
        assert_eq!(range_map.map(11), 1);

        let range_map = RangeMap::new(10, 10, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);
    }

    #[test]
    fn test_example1() {
        let range_map = RangeMap::new(98, 50, 2);
        assert_eq!(range_map.map(98), 50);
        assert_eq!(range_map.map(99), 51);
        for i in 0..98 {
            assert_eq!(range_map.map(i), i);
        }
        for i in 100..200 {
            assert_eq!(range_map.map(i), i);
        }
    }

    #[test]
    fn test_example2() {
        let range_map = RangeMap::new(50, 52, 48);
        assert_eq!(range_map.map(49), 49);
        assert_eq!(range_map.map(99), 99);
        for i in 50..98 {
            assert_eq!(range_map.map(i), i + 2);
        }
    }

    #[test]
    fn test_is_in_range() {
        let range_map = RangeMap::new(50, 52, 2);
        assert_eq!(range_map.is_in_range(49), false);
        assert_eq!(range_map.is_in_range(50), true);
        assert_eq!(range_map.is_in_range(51), true);
        assert_eq!(range_map.is_in_range(52), false);
        assert_eq!(range_map.is_in_range(53), false);
    }
}
