use rayon::prelude::*;

fn main() {
    println!("Funkcja 1:\n{:?}\n", inverse_series(f1, 10));
    println!("Funkcja 2:\n{:?}\n", inverse_series(f2, 10));
    println!("Funckja 3:\n{:?}\n", inverse_series(f3, 10));
    println!("Funckja 4:\n{:?}\n", inverse_series(f4, 10));
}

fn inverse_series<F>(f: F, n: u32) -> Vec<f64>
where
    F: Fn(u32) -> f64,
{
    let a: Vec<f64> = (0..n).map(&f).collect();
    let mut results: Vec<f64> = vec![0_f64; n as usize];
    results[0] = 1_f64 / f(0);

    for i in 1..n {
        results[i as usize] = -1_f64 / a[0]
            * results[0..(i as usize)]
                .par_iter()
                .enumerate()
                .map(|(k, b_k)| a[i as usize - k] * b_k)
                .sum::<f64>();
    }

    results
}

fn f1(_n: u32) -> f64 {
    1_f64
}

fn f2(n: u32) -> f64 {
    2_f64.powi(n as i32)
}

fn f3(n: u32) -> f64 {
    if n == 1 || n == 0 {
        1_f64
    } else {
        f3(n - 1) * n as f64
    }
}

fn f4(n: u32) -> f64 {
    1_f64 / f3(n)
}
