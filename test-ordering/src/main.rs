use std::cmp::Ordering;

fn is_number(n1: i32, n2: i32) -> Ordering {
    match n1.cmp(&n2) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
    }
}

fn main() {
    println!("{:?}", is_number(5, 3) == Ordering::Greater);
}
