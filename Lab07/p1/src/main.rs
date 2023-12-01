use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, Clone, Copy, PartialEq)]

struct Complex {
    real: f64,
    imag: f64,
}
// the structure with 2 f64 variables
impl Complex {
    fn new<U, V>(real: U, imag: V) -> Self
    where
        U: Into<f64>,
        V: Into<f64>,
    {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }
    fn conjugate(self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl From<i32> for Complex {
    fn from(real: i32) -> Self {
        Complex {
            real: real as f64,
            imag: 0.0,
        }
    }
}
impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Complex { real, imag: 0.0 }
    }
}

impl Add<Complex> for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Add<i32> for Complex {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Complex {
            real: self.real + rhs as f64,
            imag: self.imag,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}
impl Sub<Complex> for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}
impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else if self.imag < 0.0 {
            write!(f, "{}{}i", self.real, self.imag)
        } else {
            write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}
fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
