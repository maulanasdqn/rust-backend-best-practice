use fta_users::domain::User;

#[test]
fn test_user_with_empty_email() {
    let user = User::new(String::new(), "hash".to_string(), None, None);

    assert_eq!(user.email, "");
}

#[test]
fn test_user_with_very_long_email() {
    let long_email = format!("{}@example.com", "a".repeat(1000));
    let user = User::new(long_email.clone(), "hash".to_string(), None, None);

    assert_eq!(user.email, long_email);
}

#[test]
fn test_user_with_unicode_email() {
    let user = User::new("用户@例え.jp".to_string(), "hash".to_string(), None, None);

    assert_eq!(user.email, "用户@例え.jp");
}

#[test]
fn test_user_with_special_characters_in_email() {
    let user = User::new(
        "test+tag@sub.domain.co.uk".to_string(),
        "hash".to_string(),
        None,
        None,
    );

    assert_eq!(user.email, "test+tag@sub.domain.co.uk");
}

#[test]
fn test_user_with_empty_password_hash() {
    let user = User::new("user@example.com".to_string(), String::new(), None, None);

    assert_eq!(user.password_hash, "");
}

#[test]
fn test_user_with_very_long_password_hash() {
    let long_hash = "h".repeat(10000);
    let user = User::new(
        "user@example.com".to_string(),
        long_hash.clone(),
        None,
        None,
    );

    assert_eq!(user.password_hash, long_hash);
}

#[test]
fn test_user_with_empty_string_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some(String::new()),
        Some(String::new()),
    );

    assert_eq!(user.first_name, Some(String::new()));
    assert_eq!(user.last_name, Some(String::new()));
    assert_eq!(user.full_name(), Some(" ".to_string()));
}

#[test]
fn test_user_with_very_long_names() {
    let long_first = "F".repeat(5000);
    let long_last = "L".repeat(5000);
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some(long_first.clone()),
        Some(long_last.clone()),
    );

    assert_eq!(user.first_name, Some(long_first));
    assert_eq!(user.last_name, Some(long_last));
}

#[test]
fn test_user_with_unicode_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("张".to_string()),
        Some("伟".to_string()),
    );

    assert_eq!(user.full_name(), Some("张 伟".to_string()));
}

#[test]
fn test_user_with_emoji_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("John 😊".to_string()),
        Some("Doe 🎉".to_string()),
    );

    assert_eq!(user.full_name(), Some("John 😊 Doe 🎉".to_string()));
}

#[test]
fn test_user_with_special_characters_in_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("O'Brien".to_string()),
        Some("Smith-Jones".to_string()),
    );

    assert_eq!(user.full_name(), Some("O'Brien Smith-Jones".to_string()));
}

#[test]
fn test_full_name_with_whitespace_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("  John  ".to_string()),
        Some("  Doe  ".to_string()),
    );

    assert_eq!(user.full_name(), Some("  John     Doe  ".to_string()));
}

#[test]
fn test_full_name_with_only_whitespace_first_name() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("   ".to_string()),
        Some("Doe".to_string()),
    );

    assert_eq!(user.full_name(), Some("    Doe".to_string()));
}

#[test]
fn test_full_name_with_only_whitespace_last_name() {
    let user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("John".to_string()),
        Some("   ".to_string()),
    );

    assert_eq!(user.full_name(), Some("John    ".to_string()));
}
