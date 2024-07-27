use super::is_surrounded_by_n_equals::{self, is_surrounded_by_n_equals};

#[cfg(test)]
mod test;


pub fn split_at_delimiters_bracketed_by_n_equals(text: &str, q: usize) -> Vec<String> {
    let is_delimiter = | text: &str | -> bool { is_surrounded_by_n_equals(text, q) }; 

    let lines: Vec<&str> = text.lines().collect();
    let mut split_indices: Vec<usize> = vec![];

    for (idx, line) in lines.iter().enumerate() {
        if is_delimiter(line) {
            split_indices.push(idx)
        }
    }

    // Split text at delimiter lines
    let mut result: Vec<String> = vec![];
    let mut last_idx = 0;

    for &split_idx in &split_indices {
        if split_idx > last_idx {
            let chunk = &lines[last_idx..split_idx].join("\n");
            result.push(chunk.to_owned());
        }
        last_idx = split_idx + 1;
    }

    if last_idx < lines.len() {
        let chunk = &lines[last_idx..].join("\n");
        result.push(chunk.to_owned());
    }

    result
}