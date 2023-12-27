pub fn bind_result<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut vec = Vec::new();
    for item in iter {
        vec.push(item?);
    }
    Ok(vec)
}
