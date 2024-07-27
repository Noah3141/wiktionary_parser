#[cfg(test)]
mod test;


pub fn is_surrounded_by_double_equals(s: &str) -> bool {
    let string = s.trim();
    // Check if the string starts and ends with "=="
    string.starts_with("==") && string.ends_with("==") && {
        // Remove the leading and trailing "==" and check if there are no additional "=" characters at the boundaries
        let inner = &string[2..string.len()-2];
        !inner.starts_with("=") && !inner.ends_with("=") && {
            // Ensure there are no internal newline characters
            !inner.contains('\n')
        }
    }
}