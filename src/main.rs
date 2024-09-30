mod quarternion;
use quarternion::Quarternion;

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

    println!("{:?}", p.add(&q));
    println!("{:?}", p.mult(&q));
    println!("{:?}", p.dot(&q));
    println!("{:?}", p.abs());
    println!("{:?}", p.conj());
    println!("{:?}", Quarternion::id());
    println!("{:?}", Quarternion::zero());
}
