mod libs;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, CellAlignment, ContentArrangement, Table,
};

use libs::{gen_vec::gen_vec, kmp::seek_patterns};
/**
 * a -> false
 * b -> true
 */

/// a -> false
/// b -> true
fn main() {
    const ITERATIONS: u32 = 1000;
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        "Array length",
        "% ciągów w których występuje wzór \"aaa\"",
        "% ciągów w których występuje wzór \"abb\"",
        "Średnia liczba wystąpień wzoru \"aaa\"",
    ]);

    for n in 3..=50 {
        let (a, b, c) = abc(n, ITERATIONS);
        table.add_row(vec![
            format!("{}", n),
            format!("{:.2}%", a * 100.),
            format!("{:.2}%", b * 100.),
            format!("{:.2}", c),
        ]);
    }

    table
        .column_iter_mut()
        .for_each(|column| column.set_cell_alignment(CellAlignment::Center));

    println!("{table}")
}

fn abc(n: u32, iterations: u32) -> (f64, f64, f64) {
    let (mut a, mut b, mut c): (f64, f64, f64) = (0., 0., 0.);
    for _ in 0..iterations {
        let v = gen_vec(n);
        let mut matches = seek_patterns(&vec![false, false, false], &v).len();
        if matches > 0 {
            a += 1.;
            c += matches as f64;
        }
        matches = seek_patterns(&vec![false, true, true], &v).len();
        if matches > 0 {
            b += 1.;
        }
    }

    a /= iterations as f64;
    b /= iterations as f64;
    c /= iterations as f64;

    (a, b, c)
}
