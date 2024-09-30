#[derive(Debug)]
struct Quarternion {
    real: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl Quarternion {
    fn add(&self, q: Quarternion) -> Quarternion {
        Quarternion {
            real: self.real + q.real,
            i: self.i + q.i,
            j: self.j + q.j,
            k : self.k + q.k,
        }
    }

    fn mult(&self, q: Quarternion) -> Quarternion {
        Quarternion {
            real: self.real * q.real - self.i * q.i - self.j * q.j -self.k * q.k,
            i: self.real * q.i + self.i * q.real + self.j * q.k - self.k * q.j,
            j: self.real * q.j + self.j * q.real + self.k * q.i - self.i * q.k,
            k: self.real * q.k + self.k * q.real + self.i + q.j - self.j * q.i,
    }
    }

    fn conj(&self) -> Quarternion {
        Quarternion {
            real: self.real,
            i: - self.i,
            j: - self.j,
            k: - self.k,
        }
    }

    fn abs(&self) -> f64 {
        self.mult(self.conj()).real
    }

    fn id() -> Quarternion {
        Quarternion {
            real: 1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

    fn zero() -> Quarternion {
        Quarternion {
            real: 0.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

}

fn main() {
    let q = Quarternion {
        real: 1.0,
        i: 2.0,
        j: 2.0,
        k: 4.0,
    };

    let p = Quarternion {
        real: 1.5,
        i: 2.6,
        j: 2.7,
        k: 4.8,
    };


    println!("{:?}", Quarternion::zero())
}
