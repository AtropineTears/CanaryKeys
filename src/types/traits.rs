pub trait ValidateCanaryType {
    fn validate_length(&self) -> bool;
    fn validate_format(&self) -> bool;
    fn validate(&self) -> bool;
}