use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    Russian,
    Ukrainian,
    Belarusian,
}