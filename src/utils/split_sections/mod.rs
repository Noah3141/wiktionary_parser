use crate::models::{language, section_header::SectionHeader};

use super::find_header_of_size::find_header_of_size;

#[cfg(test)]
mod test;

const HEADER_START: &'static str = "\n===";
const HEADER_START_LEN: usize = HEADER_START.len();
const HEADER_END: &'static str = "===\n";
const HEADER_END_LEN: usize = HEADER_START.len();

pub fn split_sections(language_section: &str) -> Vec<(SectionHeader, &str)> {
    let matches = language_section.match_indices(HEADER_START);
    let mut sections: Vec<(SectionHeader, &str)> = Vec::new();

    for (header_index, tag_start) in matches {
        if language_section[&header_index + &HEADER_START_LEN ..].chars().next().expect("next") != '=' {
            let header_onward = &language_section[header_index..];
            let header = &header_onward[..header_onward.find(HEADER_END).unwrap() + HEADER_END_LEN];
            let section_header = match SectionHeader::try_from(header) {
                Ok(h) => h,
                Err(e) => {println!("Skipping section ({}): {e}", &language_section[..12]); continue},
            };


            let next = find_header_of_size(3, &header_onward[header.len()..]);
            match next {
                Some(next_header) => {
                    let section = &header_onward[..next_header + header.len()];
                    sections.push((section_header, section))
                },
                None => {
                    sections.push((section_header, header_onward))
                },
            }
        } else {
            continue;
        }
    }

    sections
}
