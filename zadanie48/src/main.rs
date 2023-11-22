use gnuplot::{Caption, Figure};

fn sd(n: u32) -> u32 {
    n.count_ones()
}

fn s(n: u32) -> u32 {
    (1..=n).map(sd).sum()
}

fn a(n: u32, scalar: f64) -> f64 {
    (n as f64).log2() * scalar * n as f64
}

fn main() {
    let n_values: Vec<u32> = (1..=1024).collect();
    let s_values: Vec<u32> = n_values.iter().map(|&n| s(n)).collect();
    let a_values: Vec<f64> = n_values.iter().map(|&n| a(n, 0.5)).collect();

    // Plotting s(n) and saving to PNG file
    let mut fg = Figure::new();
    fg.axes2d().lines(&n_values, &s_values, &[Caption("s(n)")]);
    let _ = fg
        .save_to_png("s_n_plot.png", 1200, 800)
        .expect("Nie zapisano 1");

    // Plotting s(n) - a(n) and saving to PNG file
    let mut fg2 = Figure::new();
    fg2.axes2d().lines(
        &n_values,
        s_values.iter().zip(&a_values).map(|(&s, &a)| s as f64 - a),
        &[Caption("s(n) - a(n)")],
    );
    let _ = fg2
        .save_to_png("s_n_a_n_plot.png", 1200, 800)
        .expect("Nie zapisano 2");
}
