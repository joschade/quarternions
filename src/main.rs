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

    println!("{}", p.clone() + q.clone());
    println!("{}", p.clone() - q.clone());
    println!("{}", p.clone() * q.clone());
    println!("{}", p.clone() / q.clone());
    println!("{}", p.clone().len());
    println!("{}", p.clone().norm());
    println!("{}", p.clone().dot(&p));
    println!("should yield 1: {}", Quarternion::id());
    println!("should yield 0: {}", Quarternion::zero());
    println!("{}", q.clone().inv());


    let r = Quarternion::new(2.0, 0.0, -7., 3.0);

    println! {"{}", r.clone().conj().normalize()}
    println! {"{:?}", r.clone().conj().normalize().get_array()}
    println! {"{:?}", r.clone().conj().normalize().get_eulerrad()}
    println! {"{:?}", r.clone().is_unit()}
    println! {"{:?}", r.clone().normalize().get_eulerdeg()}
    println! {"{}", Quarternion::from_eulerrad([1., 2., -2.])}
    println! {"{}", q.conj()}
    println!("{}", Quarternion::new(1., 1., 1., 1.)/7.);

}
