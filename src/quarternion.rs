use std::fmt;

#[derive(Clone)]
pub struct Quarternion {
    pub real: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
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
    
    pub fn add(&self, q: &Quarternion) -> Quarternion {
        Quarternion {
            real: self.real + q.real,
            i: self.i + q.i,
            j: self.j + q.j,
            k : self.k + q.k,
        }
    }

    pub fn mult(&self, q: &Quarternion) -> Quarternion {
        Quarternion {
            real: self.real * q.real - self.i * q.i - self.j * q.j - self.k * q.k,
            i: self.real * q.i + self.i * q.real + self.j * q.k - self.k * q.j,
            j: self.real * q.j + self.j * q.real + self.k * q.i - self.i * q.k,
            k: self.real * q.k + self.k * q.real + self.i * q.j - self.j * q.i,
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

    pub fn dot(&self, q: &Quarternion) -> f64 {
        self.mult(&q.conj()).real
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