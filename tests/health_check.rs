use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2Prod::configuration::get_configuration;
use zero2Prod::startup::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to expcute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Fail to read configurarion");
    let configurarion_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&configurarion_string)
        .await
        .expect("Failed to connect Postgres");
    let client = reqwest::Client::new();

    let body = "name=taegyoon&email=xerse%40hanmail.net";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_mission() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=taegyoon", "missing the mail"),
        ("email=xerse%40hanmail.net", "missing the name"),
        ("", "missiong both mail and name"),
    ];
    for (invalid_boby, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_boby)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the playload waw {}.",
            error_message
        );
    }
}
