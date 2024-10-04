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

    println!("{}", p.clone()+q.clone());
    println!("{}", p.clone()-q.clone());
    println!("{}", p.clone()*q.clone());
    println!("{}", p.clone()/q.clone());
    println!("{}", p.len());
    println!("{}", p.norm());
    println!("should yield 2: {}", q.add(&q.conj()));
    println!("should yield 1: {}", Quarternion::id());
    println!("should yield 0: {}", Quarternion::zero());
    println!("{}", q.scalar_mult(4.0));
    println!("{}", q.inv());
    println!("should yield 1: {:}", q.mult(&(q.inv())));

    let r = Quarternion::new(2.0, 0.0, -7., 3.0);

    println!("{}", r.mult(&(r.conj())));
    println!("{}", r.conj().mult(&r));
    println!{"{}", r.conj().normalize()}
    println!{"{:?}", r.conj().normalize().get_array()}
    print!("{}", -r.clone());

}
