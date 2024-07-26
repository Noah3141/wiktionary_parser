use crate::models::{language, section_header::SectionHeader};

use super::find_header_of_size::find_header_of_size;

#[cfg(test)]
mod test;

const HEADER_START: &'static str = "\n===";
const HEADER_START_LEN: usize = HEADER_START.len();
const HEADER_END: &'static str = "===\n";
const HEADER_END_LEN: usize = HEADER_START.len();

pub fn split_section(language_section: &str) -> Vec<(SectionHeader, &str)> {

    let matches = language_section.match_indices(HEADER_START);
    let mut sections: Vec<(SectionHeader, &str)> = Vec::new();

    for (index, _) in matches {
        if language_section[&index + &HEADER_START_LEN..].chars().next().expect("next") != '=' {
            let header_onward = &language_section[index..];
            let header = &header_onward[.. header_onward.find(HEADER_END).unwrap() + HEADER_END_LEN];
            let section_header = SectionHeader::from(header);

            let next = find_header_of_size(3, &language_section[index + 1..]).unwrap_or(language_section.len());
            let section = &language_section[index..next];
            sections.push((section_header, section))
        } else {
            continue;
        }
    }

    sections
}
