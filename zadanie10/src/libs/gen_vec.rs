use rand;
use rayon::prelude::*;

pub fn gen_vec(size: u32) -> Vec<bool> {
    let mut v = vec![false; size as usize];
    v.par_iter_mut().for_each(|value| {
        let temp: u32 = rand::random::<u32>() % 2;
        *value = match temp {
            0 => false,
            1 => true,
            _ => panic!("Zła wartość"),
        };
    });
    v
}
