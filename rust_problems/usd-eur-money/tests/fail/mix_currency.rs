use usd_eur_money::{double_usd, EUR};

fn main() {
    let eur = EUR(10.0);
    double_usd(eur); // Waiting error on this line
}