mod libs;

use libs::static_points;

use crate::libs::{gen_vec, Wka};

fn main() {
    const N: u32 = 1000;
    abc()
}

fn abc(n: u32, iterations: u32, avg_size: u32) -> (f64, f64, f64) {
    let (mut a, mut b, mut c): (u32, u32, u32);
    let (mut avg_a, mut avg_b, mut avg_c) = (0., 0., 0.);
    for i in 0..iterations {
        for j in 0..avg_size {
            (a, b, c) = (0, 0, 0);

            let v = gen_vec(n);
            match static_points(v) {
                
            };
        }
    }

    (avg_a, avg_b, avg_c)
}