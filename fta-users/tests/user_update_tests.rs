use fta_users::domain::User;

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
