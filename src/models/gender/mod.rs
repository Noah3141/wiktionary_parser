#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}