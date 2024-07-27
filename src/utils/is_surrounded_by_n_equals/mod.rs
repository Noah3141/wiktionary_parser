use std::iter::repeat;

#[cfg(test)]
mod test;
/// What do I ACTUALLY want the edge cases to be of this crap
pub fn is_surrounded_by_n_equals(s: &str, q: usize) -> bool {
    let equals = repeat('=').take(q).collect::<String>();
    let string = s.trim();

    if string.chars().filter(|&c| c == '=').count() != (q * 2) {return false} // This is cheating
    if string.chars().filter(|&c| c != '=' && c != '\n' && c != ' ').count() == 0 {return false} // This is cheating too


    string.starts_with(&equals) && string.ends_with(&equals) && {

        [0..q].iter().enumerate().all(|(i, _)| {
            let inner = &string[i..string.len()-i];

            inner.starts_with("=") && inner.ends_with("=") && !inner.contains('\n')
        })
    }
}