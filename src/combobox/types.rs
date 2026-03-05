use pwd_types::PasswordPreset;

#[derive(Clone, PartialEq, Copy)]
pub enum AnyPreset {
    Standard(PasswordPreset),
    Custom,
}