use assert_cmd::prelude::*;
use reqwest::blocking::get;
use std::fs;
use std::process::Command;

// This function will be executed by the `cargo test` command.
#[test]
fn test_hello_path() {
    // We know that a request to /hello should return the contents of hello.html.
    let expected_response =
        fs::read_to_string(format!("{}/public/hello.html", env!("CARGO_MANIFEST_DIR"))).unwrap();

    // The `cargo_bin` function will find the executable for the `server` crate and the `spawn` method will run it in the background.
    let mut test_bin = Command::cargo_bin("server").unwrap().spawn().unwrap();

    // We make a GET request to the /hello path on the server. The `text` method will return the response body as a String.
    let response = get("http://127.0.0.1:8080/hello").unwrap().text().unwrap();

    // We kill the server before we exit the test
    test_bin.kill().unwrap();

    // The `assert_eq` macro will assert if both parameters are the same.
    // In this case the response body should be the same as the contents of the hello.html file.
    assert_eq!(expected_response, response);
}
