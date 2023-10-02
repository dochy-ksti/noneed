use rand_distr::Normal;

#[test]
fn hoge() {
    use rand::prelude::*;
    use rand_distr::StandardNormal;

    let normal = Normal::new(0., 2.).unwrap();
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for _ in 0..1000 {
        let val: f64 = thread_rng().sample(normal);
        if val < min {
            min = val;
        }
        if max < val {
            max = val;
        }
        println!("{}", val);
    }
    println!("min {min} max {max}")
}
