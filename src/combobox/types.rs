use pwd_types::PasswordPreset;

#[derive(Clone, PartialEq)]
enum AnyPreset {
    Standard(PasswordPreset),
    Custom,
}