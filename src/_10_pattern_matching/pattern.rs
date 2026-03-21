pub fn pattern(a: i32) -> Result<(), ()> {
    match a {
        1 => Ok(()),
        2 => Ok(()),
        _ => Err(()),
    }
}