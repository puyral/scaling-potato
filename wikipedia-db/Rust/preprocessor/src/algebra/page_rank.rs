use std::io;
use std::io::Write;
use std::ops::Add;

use sprs::{CsMatI, CsVecI};
use termion::{clear, color, style};

/// Applies the PageRank algorithm:
///
/// Find the limit of
///  $$ \pi_{n+1}=\beta*m\pi_n+\frac{1-\beta}{n}\mathbf{1}^1 $$
///
/// Here `m` is $m$, `pi_ref` is basically $\pi_0$ except its relevant coefficients are all at `1.0`
/// and the function figures out what value to give them by itself. Finally, `epsilon` tells us when to stop
pub fn page_rank(
    m: &CsMatI<f64, u32>,
    pi_ref: &CsVecI<f64, u32>,
    beta: f64,
    epsilon: f64,
) -> CsVecI<f64, u32> {
    println!("\t=> {} categories and {} links", pi_ref.nnz(), m.nnz());
    let n = pi_ref.nnz() as f64;

    let mut pi_new = pi_ref.map(|&d| d * 1.0 / n);
    let mut pi_old; //= CsVecI::new(pi_ref.dim(), Vec::new(), Vec::new());

    let (bm, bv) = (beta, (1.0 - beta) / n);
    let pi_added = pi_ref.map(|&d| d * bv);

    let mut diff = 1.0;
    let mut i = 0;
    while diff > epsilon {
        // so the epsilon doesn't depends on size of the graph

        print!(
            "\r{clear}\tnorm: {},\tdiff: {}",
            pi_new.l1_norm(),
            diff,
            clear = clear::CurrentLine
        );
        io::stdout().flush().ok().expect("Could not flush stdout");
        pi_old = pi_new.map(|&d| -d);
        // pi_new = csr_mul_csvec(m.view(), pi_new.view()).map(|&d| d * bm).add(&pi_added);
        pi_new = (m * &pi_new).map(|&d| d * bm) + &pi_added;
        diff = pi_old.add(&pi_new).l1_norm();

        i += 1;
    }
    println!(
        "\r{clear}\tnorm: {}\n\tdiff: {}\n\t{green}{bold}[DONE]{reset_c}{reset_s} (in {} loops)",
        pi_new.l1_norm(),
        diff,
        i,
        clear = clear::CurrentLine,
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );
    pi_new
}
