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
    println!("{}", p.clone().len());
    println!("{}", p.clone().norm());
    println!("{}", p.clone().dot(p.clone()));
    println!("should yield 1: {}", Quarternion::id());
    println!("should yield 0: {}", Quarternion::zero());
    println!("{}", q.clone().scalar_mult(4.0));
    println!("{}", q.clone().inv());

    let r = Quarternion::new(2.0, 0.0, -7., 3.0);

    println!{"{}", r.conj().normalize()}
    println!{"{:?}", r.conj().normalize().get_array()}

}
