// Week 14 — validator.rs

#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Medium => "Medium",
            PasswordStrength::Strong => "Strong",
            PasswordStrength::VeryStrong => "Very strong",
        };
        write!(f, "{}", label)
    }
}

pub fn validate_strength(password: &str) -> PasswordStrength {
    let mut score = 0;

    let length = password.len();

    if length >= 8 {
        score += 1;
    }
    if length >= 12 {
        score += 1;
    }
    if length >= 16 {
        score += 1;
    }

    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());

    if has_lower {
        score += 1;
    }
    if has_upper {
        score += 1;
    }
    if has_digit {
        score += 1;
    }
    if has_symbol {
        score += 1;
    }

    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Medium,
        5..=6 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

pub fn check_common_patterns(password: &str) -> bool {
    let lower = password.to_lowercase();

    // Check if all characters are the same
    if password
        .chars()
        .all(|c| c == password.chars().next().unwrap_or('\0'))
    {
        return true;
    }

    // Check against common passwords (case-insensitive)
    COMMON_PASSWORDS.iter().any(|&p| p == lower)
}

pub fn calculate_entropy(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0;
    }

    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());

    let mut charset_size = 0;

    if has_lower {
        charset_size += 26;
    }
    if has_upper {
        charset_size += 26;
    }
    if has_digit {
        charset_size += 10;
    }
    if has_symbol {
        charset_size += 32; // brings total to 94
    }

    let length = password.len() as f64;

    length * (charset_size as f64).log2()
}

pub const COMMON_PASSWORDS: &[&str] = &[
    "password",
    "123456",
    "password123",
    "qwerty",
    "letmein",
    "iloveyou",
    "admin",
    "welcome",
    "monkey",
    "dragon",
];
