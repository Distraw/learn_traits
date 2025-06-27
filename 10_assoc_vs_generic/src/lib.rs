trait Power<T> {
    fn power(&self, value: T) -> u32;
}

impl Power<u16> for u32 {
    fn power(&self, value: u16) -> u32 {
        let mut result: u32 = 1;
        for _ in 0..value {
            result *= *self;
        }

        result
    }
}

impl Power<u32> for u32 {
    fn power(&self, value: u32) -> u32 {
        let mut result: u32 = 1;
        for _ in 0..value {
            result *= *self;
        }

        result
    }
}

impl Power<&u32> for u32 {
    fn power(&self, value: &u32) -> u32 {
        let mut result: u32 = 1;
        for _ in 0..*value {
            result *= *self;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
