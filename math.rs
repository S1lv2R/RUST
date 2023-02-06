#[derive(Debug, Copy, Clone)]
struct Fraction {
    numerator: i64,
    denominator: i64
}

pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
}

impl Fraction {
    pub fn input(&mut self, number: String) {
        let array: Vec<String> = number.split('/').map(|s| s.to_string()).collect();
        assert!(!array.is_empty());

        let numerator: i64 = array[0].parse().unwrap();
        let denominator: i64 = if array.len() > 1 { array[1].parse().unwrap() } else { 1 };
        
        assert!(denominator != 0);

        self.numerator = numerator;
        self.denominator = denominator;
    }

    pub fn simplify(mut fraction: Self) -> Fraction {
        let GCD: i64 = gcd(fraction.numerator, fraction.denominator);
        fraction.numerator /= GCD;
        fraction.denominator /= GCD;

        return fraction;
    }
}

impl std::ops::Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        return Fraction::simplify(Fraction {
            numerator: (self.numerator * other.numerator) + (self.denominator * other.denominator),
            denominator: self.denominator * other.denominator
        });
    }
}

impl std::ops::Sub<Fraction> for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        return Fraction::simplify(Fraction {
            numerator: (self.numerator * other.numerator) - (self.denominator * other.denominator),
            denominator: self.denominator * other.denominator
        });
    }
}

impl std::ops::Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        return Fraction::simplify(Fraction {
            numerator: (self.numerator * other.numerator),
            denominator: self.denominator * other.denominator
        });
    }
}

impl std::ops::Div<i64> for Fraction {
    type Output = Fraction;

    fn div(self, scalar: i64) -> Fraction {
        return Fraction::simplify(Fraction {
            numerator: (self.numerator * scalar),
            denominator: self.denominator * scalar
        });
    }
}

fn main() {
    let mut b: Fraction = Fraction { numerator: 0, denominator: 0 };
    b.input(String::from("3/4"));
    println!("{:#?}", b);
}
