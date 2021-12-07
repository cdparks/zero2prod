use crate::app;

#[actix_rt::test]
async fn should_200_for_valid_form() {
    let app = app::spawn().await;

    let response = reqwest::Client::new()
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("failed to fetch saved subscription");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn should_400_on_missing_data() {
    let app = app::spawn().await;
    let client = reqwest::Client::new();

    let cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error) in cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "did not fail with 400 Bad Request despite {}",
            error
        );
    }
}
