use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Snafu(pub Vec<i8>);

impl Snafu {
    pub fn digit_to_char(digit: i8) -> char {
        match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("Unexpected SNAFU digit (i8): {digit}"),
        }
    }
    
    pub fn char_to_digit(c: char) -> i8 {
        match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unexpected SNAFU digit (char): {c:?}"),
        }
    }

    pub fn parse(s: &str) -> Snafu {
        let vec = s
            .chars()
            .map(Self::char_to_digit)
            .rev()
            .collect();

        Snafu(vec)
    }

    pub fn to_decimal(&self) -> i64 {
        let mut w = 1;
        let mut ret = 0;
        for &digit in &self.0 {
            ret += w * digit as i64;
            w *= 5;
        }
        ret
    }

    pub fn from_decimal(num: i64) -> Snafu {
        let mut digits = Self::to_base_5(num);
        let carry = Self::add_2_with_carry(&mut digits);
        for d in digits.iter_mut() {
            *d -= 2;
        }
        if carry != 0 {
            assert_eq!(carry, 1);
            digits.push(carry);
        }
        Snafu(digits)
    }

    fn to_base_5(mut num: i64) -> Vec<i8> {
        assert!(num >= 0);
        let mut ret = Vec::new();
        while num > 0 {
            ret.push((num % 5) as i8);
            num /= 5;
        }
        ret
    }

    fn add_2_with_carry(digits: &mut [i8]) -> i8 {
        let mut carry = 0;
        for d in digits.iter_mut() {
            *d = *d + 2 + carry;
            carry = *d / 5;
            *d %= 5;
        }
        carry
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in self.0.iter().rev() {
            write!(f, "{}", Snafu::digit_to_char(digit))?;
        }
        Ok(())
    }
}

fn main() {
    let content = std::fs::read_to_string("input2").unwrap();

    let sum = content
        .lines()
        .map(|line| Snafu::parse(line).to_decimal())
        .sum::<i64>();

    println!("sum (decimal): {sum}");
    println!("sum (SNAFU): {}", Snafu::from_decimal(sum));
}
