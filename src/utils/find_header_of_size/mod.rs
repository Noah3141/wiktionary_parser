#[cfg(test)]
mod test;

/// `size` determines how many = together make up the header bracket
/// Returns the index of the leading newline in `\n========Title=========`
pub fn find_header_of_size(size: usize, text: &str) -> Option<usize> {
    let header = "\n".to_string() + &std::iter::repeat('=').take(size).collect::<String>();
    let matches = text.match_indices(&header);

    for (index, _) in matches {
        if text[index + (size + 1)..].chars().next().expect("next") != '=' {
            return Some(index)
        } else { continue }
    }

    None
}