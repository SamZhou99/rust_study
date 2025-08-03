use std::fmt;
#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn test_min_max() {
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "big:{big}, small:{small}",
        small = small_range,
        big = big_range
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_max() {
        let minmax = MinMax(0, 14);
        assert_eq!(format!("{}", minmax), "(0, 14)");
    }
}
