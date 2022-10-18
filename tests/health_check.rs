use std::net::TcpListener;

#[tokio::test]
async fn health_check_should_return_ok() {
    // setup
    let address = spawn_app();

    // reqwest is used to decouple the project from
    // one framework
    //
    // if we would want to change actix web to something else
    // the test suite written with actix methods
    // would be useless
    let client = reqwest::Client::new();

    // perform test action
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert that everything works fine
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // trying to bind port 0 will trigger OS search for available ports
    // Retrieve a port given by the OS
    let port = listener.local_addr().unwrap().port();

    let server = zero2production::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
