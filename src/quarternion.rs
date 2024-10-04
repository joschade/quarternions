use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone)]
pub struct Quarternion {
    pub real: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Add for Quarternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {real: self.real + other.real
            , i: self.i + other.i
            , j: self.j + other.j
            , k: self.k + other.k}
    }
}

impl Sub for Quarternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {real: self.real - other.real
            , i: self.i - other.i
            , j: self.j - other.j
            , k: self.k - other.k}
    }
}

impl Mul for Quarternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.real * other.i + self.i * other.real + self.j * other.k - self.k * other.j,
            j: self.real * other.j + self.j * other.real + self.k * other.i - self.i * other.k,
            k: self.real * other.k + self.k * other.real + self.i * other.j - self.j * other.i,
        }
    }
}

impl Neg for Quarternion {
    type Output = Self;

    fn neg(self) -> Self {
        Self {real: - self.real
            , i: - self.i
            , j: - self.j
            , k: - self.k }
    }
}

impl Quarternion {
    pub fn new(w: f64, x:f64, y:f64, z:f64) -> Quarternion {
        Quarternion {
            real: w,
            i: x,
            j: y,
            k: z
        }
    }
    
    pub fn add(&self, other: &Quarternion) -> Quarternion {
        Quarternion {
            real: self.real + other.real,
            i: self.i + other.i,
            j: self.j + other.j,
            k : self.k + other.k,
        }
    }

    pub fn mult(&self, other: &Quarternion) -> Quarternion {
        Quarternion {
            real: self.real * other.real - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.real * other.i + self.i * other.real + self.j * other.k - self.k * other.j,
            j: self.real * other.j + self.j * other.real + self.k * other.i - self.i * other.k,
            k: self.real * other.k + self.k * other.real + self.i * other.j - self.j * other.i,
    }
    }

    pub fn conj(&self) -> Quarternion {
        Quarternion {
            real: self.real,
            i: - self.i,
            j: - self.j,
            k: - self.k,
        }
    }

    pub fn dot(&self, other: &Quarternion) -> f64 {
        self.mult(&other.conj()).real

    }

    // norm in this context means squared length
    pub fn norm(&self) -> f64 {
        self.dot(self)
    }

    pub fn len(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn id() -> Quarternion {
        Quarternion {
            real: 1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

    pub fn zero() -> Quarternion {
        Quarternion {
            real: 0.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

    pub fn scalar_mult(&self, x: f64) -> Quarternion {
        Quarternion {
            real: self.real*x,
            i: self.i*x,
            j: self.j*x,
            k: self.k*x,
        }
    }

    pub fn inv(&self) -> Quarternion {
        let norm = self.norm();

        Quarternion::scalar_mult(&self.conj(), 1./norm)
    }

    pub fn normalize(&self) -> Quarternion {
        self.scalar_mult(1./self.len())
        }

    pub fn get_array(&self) -> [f64; 4] {
        [self.real, self.i, self.j, self.k]
    }
}

impl Div for Quarternion {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

// helper for fmt::Display
fn sgn(x: f64) -> char {
    if x.signum() < 0. {'-'}
    else {'+'}
}

// format instruction for print macro
impl fmt::Display for Quarternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}i{}{}j{}{}k", self.real, sgn(self.i), self.i.abs(),sgn(self.i), self.j.abs(), sgn(self.k), self.k.abs())
    }
}