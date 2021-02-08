fn main() {
    let args = std::env::args().nth(1);
    let year = args.expect("I wasn't given an argument!").parse::<u64>().ok().expect("I wasn't given an integer!");
    if is_leap_year(year) {
        println!("{} IS a leap year", year);
    } else {
        println!("{} is NOT a leap year", year);
    }
}

pub fn is_leap_year(year: u64) -> bool {
    year.is_divisible_by(400)
        || (
        year.is_divisible_by(4)
            && year.is_not_divisible_by(100)
    )
}

trait Divisible {
    fn is_divisible_by(&self, divisor: u64) -> bool;
    fn is_not_divisible_by(&self, divisor: u64) -> bool;
}

impl Divisible for u64 {
    fn is_divisible_by(&self, divisor: u64) -> bool {
        self % divisor == 0
    }

    fn is_not_divisible_by(&self, divisor: u64) -> bool {
        !self.is_divisible_by(divisor)
    }
}