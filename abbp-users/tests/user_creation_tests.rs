use abbp_users::domain::User;

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
fn test_email_is_preserved() {
    let user = User::new(
        "test@example.com".to_string(),
        "hash".to_string(),
        None,
        None,
    );

    assert_eq!(user.email, "test@example.com");
}
