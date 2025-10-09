use fta_users::domain::User;

#[test]
fn test_new_creates_user_with_correct_fields() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    assert_eq!(user.email, "user@example.com");
    assert_eq!(user.password_hash, "hashed_password");
    assert_eq!(user.first_name, Some("John".to_string()));
    assert_eq!(user.last_name, Some("Doe".to_string()));
}

#[test]
fn test_new_user_with_none_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        None,
        None,
    );

    assert!(user.first_name.is_none());
    assert!(user.last_name.is_none());
}

#[test]
fn test_new_user_timestamps_are_set() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        None,
        None,
    );

    assert_eq!(user.created_at, user.updated_at);
}

#[test]
fn test_full_name_with_both_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    assert_eq!(user.full_name(), Some("John Doe".to_string()));
}

#[test]
fn test_full_name_with_first_name_only() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("John".to_string()),
        None,
    );

    assert_eq!(user.full_name(), Some("John".to_string()));
}

#[test]
fn test_full_name_with_last_name_only() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        None,
        Some("Doe".to_string()),
    );

    assert_eq!(user.full_name(), Some("Doe".to_string()));
}

#[test]
fn test_full_name_with_no_names() {
    let user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        None,
        None,
    );

    assert_eq!(user.full_name(), None);
}

#[test]
fn test_update_profile_changes_both_names() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("Old First".to_string()),
        Some("Old Last".to_string()),
    );

    user.update_profile(Some("New First".to_string()), Some("New Last".to_string()));

    assert_eq!(user.first_name, Some("New First".to_string()));
    assert_eq!(user.last_name, Some("New Last".to_string()));
}

#[test]
fn test_update_profile_can_set_to_none() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    user.update_profile(None, None);

    assert!(user.first_name.is_none());
    assert!(user.last_name.is_none());
}

#[test]
fn test_update_profile_updates_timestamp() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hashed_password".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    let original_updated_at = user.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    user.update_profile(Some("Jane".to_string()), Some("Doe".to_string()));

    assert!(user.updated_at > original_updated_at);
}

#[test]
fn test_change_password_updates_hash() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "old_hash".to_string(),
        None,
        None,
    );

    user.change_password("new_hash".to_string());

    assert_eq!(user.password_hash, "new_hash");
}

#[test]
fn test_change_password_updates_timestamp() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "old_hash".to_string(),
        None,
        None,
    );

    let original_updated_at = user.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    user.change_password("new_hash".to_string());

    assert!(user.updated_at > original_updated_at);
}

#[test]
fn test_email_is_preserved() {
    let user = User::new(
        "test@example.com".to_string(),
        "hash".to_string(),
        None,
        None,
    );

    assert_eq!(user.email, "test@example.com");
}

#[test]
fn test_multiple_profile_updates() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        None,
        None,
    );

    user.update_profile(Some("First".to_string()), Some("Last".to_string()));
    user.update_profile(Some("Second".to_string()), Some("Name".to_string()));

    assert_eq!(user.first_name, Some("Second".to_string()));
    assert_eq!(user.last_name, Some("Name".to_string()));
}

#[test]
fn test_update_profile_first_name_only() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    user.update_profile(Some("Jane".to_string()), Some("Doe".to_string()));

    assert_eq!(user.first_name, Some("Jane".to_string()));
    assert_eq!(user.last_name, Some("Doe".to_string()));
}

#[test]
fn test_update_profile_last_name_only() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    user.update_profile(Some("John".to_string()), Some("Smith".to_string()));

    assert_eq!(user.first_name, Some("John".to_string()));
    assert_eq!(user.last_name, Some("Smith".to_string()));
}

#[test]
fn test_full_name_after_profile_update() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        None,
        None,
    );

    user.update_profile(Some("John".to_string()), Some("Doe".to_string()));

    assert_eq!(user.full_name(), Some("John Doe".to_string()));
}

// Edge case tests
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
    assert_eq!(user.full_name(), Some(" ".to_string())); // Empty strings result in space
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
fn test_update_profile_to_empty_strings() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    user.update_profile(Some(String::new()), Some(String::new()));

    assert_eq!(user.first_name, Some(String::new()));
    assert_eq!(user.last_name, Some(String::new()));
}

#[test]
fn test_change_password_to_empty_string() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "original_hash".to_string(),
        None,
        None,
    );

    user.change_password(String::new());
    assert_eq!(user.password_hash, "");
}

#[test]
fn test_multiple_password_changes() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "hash1".to_string(),
        None,
        None,
    );

    user.change_password("hash2".to_string());
    user.change_password("hash3".to_string());
    user.change_password("hash4".to_string());

    assert_eq!(user.password_hash, "hash4");
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
