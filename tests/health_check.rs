#[tokio::test]
async fn health_check_should_return_ok() {
    // setup
    spawn_app();

    // reqwest is used to decouple the project from
    // one framework
    //
    // if we would want to change actix web to something else
    // the test suite written with actix methods
    // would be useless
    let client = reqwest::Client::new();

    // perform test action
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert that everything works fine
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2production::run().expect("Failed to bind address");

    // thanks to the refactoring of the run() function
    // spawn_app does not have to be async
    let _ = tokio::spawn(server);
}
