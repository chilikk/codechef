fn main() -> std::io::Result<()> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    println!("{}", line.trim().parse::<i64>().unwrap());
    Ok(())
}
