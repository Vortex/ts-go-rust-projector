fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(7);
    }

    return Ok(());
}

fn main() -> Result<(), usize> {
    error_me(false)?;

    let value = match error_me(false) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };

    println!("Error value: {:?}", value);

    return Ok(());
}
