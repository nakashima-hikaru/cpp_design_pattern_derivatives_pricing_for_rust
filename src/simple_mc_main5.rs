mod chapter1;
mod chapter3;
fn main() {
    use crate::chapter3::double_digital;
    use crate::chapter3::payoff2;

    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter low barrier\n");
    let low = text_io::read!();

    println!("\nEnter up barrier\n");
    let up = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol = text_io::read!();

    println!("\nEnter r\n");
    let r = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let the_payoff = double_digital::PayoffDoubleDigital::new(low, up);

    let result = <dyn payoff2::Payoff>::simple_montecarlo2(
        &the_payoff,
        expiry,
        spot,
        vol,
        r,
        number_of_paths,
    );

    println!("the price is {} \n", result);
}