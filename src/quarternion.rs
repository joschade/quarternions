use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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
        Self {
            real: self.real + other.real,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl Neg for Quarternion {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            real: -self.real,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

impl Sub for Quarternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
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

impl Div for Quarternion {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl Quarternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quarternion {
        Quarternion {
            real: w,
            i: x,
            j: y,
            k: z,
        }
    }

    pub fn conj(self) -> Quarternion {
        Quarternion {
            real: self.real,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    pub fn dot(self, other: Quarternion) -> f64 {
        (self * other.conj()).real
    }

    // norm in this context means squared length
    pub fn norm(self) -> f64 {
        self.clone().dot(self.clone())
    }

    pub fn len(&self) -> f64 {
        self.clone().norm().sqrt()
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

    pub fn scalar_mult(self, x: f64) -> Quarternion {
        Quarternion {
            real: self.real * x,
            i: self.i * x,
            j: self.j * x,
            k: self.k * x,
        }
    }

    pub fn inv(self) -> Quarternion {
        let norm = self.clone().norm();

        Quarternion::scalar_mult(self.conj(), 1. / norm)
    }

    pub fn normalize(self) -> Quarternion {
        self.clone().scalar_mult(1. / self.len())
    }

    pub fn get_array(&self) -> [f64; 4] {
        [self.real, self.i, self.j, self.k]
    }

    pub fn is_unit(&self) -> bool {
        if (self.len() != 1.0) {
            return false;
        } else {
            true
        }
    }

    pub fn get_eulerrad(&self) -> [f64; 3] {
        //returns Euler angles in radians. Only use with unit quarternions!

        if self.is_unit() {} else {
            println!("WARNING: Check whether unit quarternion is used to compute angles. \
            Length should be 1, is {}!", self.len())
        }
        [
            (2. * (self.real * self.i + self.j * self.k))
                .atan2(1. - 2. * (self.real.powi(2) + self.j.powi(2))),
            (2. * (self.real * self.j - self.i * self.k)).asin(),
            (2. * (self.real * self.j + self.i * self.k))
                .atan2(1. - 2. * (self.k.powi(2) + self.j.powi(2))),
        ]
    }

    pub fn get_eulerdeg(&self) -> [f64; 3] {
        //returns Euler angles in degrees. Only use with unit quarternions!
        let rads = self.get_eulerrad();
        let mut degs: [f64; 3] = [0., 0., 0.];
        for i in 0..3 {
            degs[i] = rads[i] * 360. * std::f64::consts::FRAC_1_PI
        }
        degs
    }

    pub fn from_eulerrad(arr: [f64; 3]) -> Quarternion {
        let alpha_q = Quarternion::new((arr[0] / 2.).cos(), 0., 0., (arr[0] / 2.).sin());
        let beta_q = Quarternion::new((arr[1] / 2.).cos(), 0., (arr[1] / 2.).sin(), 0.);
        let gamma_q = Quarternion::new((arr[2] / 2.).cos(), (arr[2] / 2.).sin(), 0., 0.);

        alpha_q * beta_q * gamma_q
    }

    pub fn from_eulerdeg(arr: [f64; 3]) -> Quarternion {
        let mut rads = arr.clone();
        for i in 0..3 {
            rads[i] = rads[i] * std::f64::consts::PI / 360.
        }
        Quarternion::from_eulerrad(rads)
    }
}

// helper for fmt::Display
fn sgn(x: f64) -> char {
    if x.signum() < 0. {
        '-'
    } else {
        '+'
    }
}

// format instruction for print macro
impl fmt::Display for Quarternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}i{}{}j{}{}k",
            self.real,
            sgn(self.i),
            self.i.abs(),
            sgn(self.i),
            self.j.abs(),
            sgn(self.k),
            self.k.abs()
        )
    }
}
