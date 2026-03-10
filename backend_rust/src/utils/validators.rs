use validator::ValidationError;

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::new("username_too_short"));
    }
    if username.len() > 32 {
        return Err(ValidationError::new("username_too_long"));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::new("username_invalid_chars"));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    if password.len() > 128 {
        return Err(ValidationError::new("password_too_long"));
    }
    Ok(())
}
