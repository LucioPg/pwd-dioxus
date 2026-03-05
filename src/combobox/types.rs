use pwd_types::PasswordPreset;

#[derive(Clone, PartialEq)]
pub enum AnyPreset {
    Standard(PasswordPreset),
    Custom,
}