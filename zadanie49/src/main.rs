use gnuplot::{Caption, Figure};

fn pn(n: u32) -> f64 {
    let mut pns: Vec<f64> = vec![1_f64];

    (1..=n).into_iter().for_each(|j| {
        pns.push(pns[..j as usize].iter().map(|pj| sigma(j) as f64 * pj).sum::<f64>() / j as f64);
    });

    pns[n as usize - 1] / n as f64
}

fn sigma(n: u32) -> u32 {
    (1..=n).into_iter().filter(|x| n % x == 0).sum()
}

fn main() {
    let n_values: Vec<u32> = (1..=100).collect();
    let s_values: Vec<f64> = n_values.iter().map(|&n| pn(n)).collect();
    for value in s_values.iter().enumerate() {
        println!("{} = {}", value.0 + 1, value.1);
    }
    // Plotting s(n) and saving to PNG file
    let mut fg = Figure::new();
    fg.axes2d().lines(&n_values, &s_values, &[Caption("s(n)")]);
    let _ = fg
        .save_to_png("s_n_plot.png", 1200, 800)
        .expect("Nie zapisano 1");
}
