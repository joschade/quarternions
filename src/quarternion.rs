#[derive(Debug, Clone)]
pub struct Quarternion {
    pub real: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Quarternion {
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
            real: self.real * q.real - self.i * q.i - self.j * q.j -self.k * q.k,
            i: self.real * q.i + self.i * q.real + self.j * q.k - self.k * q.j,
            j: self.real * q.j + self.j * q.real + self.k * q.i - self.i * q.k,
            k: self.real * q.k + self.k * q.real + self.i + q.j - self.j * q.i,
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

    pub fn abs(&self) -> f64 {
        self.dot(self)
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
}