mod libs;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, CellAlignment, ContentArrangement, Table,
};
use libs::{cycles, gen_vec, static_points, Wka};

fn main() {
    const ITERATIONS: u32 = 1000;
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        "Array length",
        "Średnia liczba permutacji bez stałych pkt",
        "Średnia liczba permutacji z jednym stałym pkt",
        "Średnia liczba cykli permutacji",
    ]);

    for n in (100..1000).step_by(100) {
        let (a, b, c) = abc(n, ITERATIONS);
        table.add_row(vec![
            format!("{}", n),
            format!("{:.2}%", a * 100),
            format!("{:.2}%", b * 100),
            format!("{:.2}", c),
        ]);
    }
    for n in (1000..10000).step_by(1000) {
        let (a, b, c) = abc(n, ITERATIONS);
        table.add_row(vec![
            format!("{}", n),
            format!("{:.2}%", a * 100),
            format!("{:.2}%", b * 100),
            format!("{:.2}", c),
        ]);
    }
    // for n in (10000..=100000).step_by(10000) {
    //     let (a, b, c) = abc(n, ITERATIONS);
    //     table.add_row(vec![
    //         format!("{}", n),
    //         format!("{:.2}%", a * 100),
    //         format!("{:.2}%", b * 100),
    //         format!("{:.2}", c),
    //     ]);
    // }

    table
        .column_iter_mut()
        .for_each(|column| column.set_cell_alignment(CellAlignment::Center));

    println!("{table}")
}

fn abc(n: u32, iterations: u32) -> (f64, f64, f64) {
    let (mut a, mut b, mut c): (f64, f64, f64) = (0., 0., 0.);
    for _ in 0..iterations {
        let mut v = gen_vec(n);
        v.shuffle();

        match static_points(&v) {
            0 => a += 1.,
            1 => b += 1.,
            _ => {}
        };
        c += cycles(&v) as f64;
    }

    a /= iterations as f64;
    b /= iterations as f64;
    c /= iterations as f64;

    (a, b, c)
}
