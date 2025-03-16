fn main() -> Result<(), std::io::Error> {
    println!("What is you name?");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;

    println!("Hello, {}", name.trim());

    Ok(())
}
