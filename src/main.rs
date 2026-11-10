use rust_for_ai::LinearRegression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Train on points following y = 2x + 1.
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 3.0, 5.0, 7.0, 9.0];

    let model = LinearRegression::fit(&x, &y)?;
    println!("Rust for AI — linear regression");
    println!("Slope: {}", model.slope);
    println!("Intercept: {}", model.intercept);
    println!("Prediction for x=6: {}", model.predict(6.0));
    Ok(())
}
