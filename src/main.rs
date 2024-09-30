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


    println!("{:?}", q.add(p))
}
