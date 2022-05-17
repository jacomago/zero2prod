use crate::helpers::{spawn_app, assert_is_redirect_to};

#[tokio::test]
async fn newsletters_page_is_not_empty() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // Act - Part 1 - Login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Part 2 - Get Send Newsletter Page
    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains(r#"Send a Newsletter"#));

    // Act - Part 3 - Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
}
