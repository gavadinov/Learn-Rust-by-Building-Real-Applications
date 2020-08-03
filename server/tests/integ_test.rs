use assert_cmd::prelude::*;
use reqwest::blocking::get;
use std::fs;
use std::process::{Child, Command};

struct ProcessGuard {
    process: Child,
}

impl ProcessGuard {
    fn new(process: Child) -> Self {
        Self { process }
    }
}

impl Drop for ProcessGuard {
    // The Drop method is the destructor for the struct. It will be invoked when the struct is destroyed.
    // https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

// This function will be executed by the `cargo test` command.
#[test]
fn test_hello_path() {
    // We know that a request to /hello should return the contents of hello.html.
    let expected_response =
        fs::read_to_string(format!("{}/public/hello.html", env!("CARGO_MANIFEST_DIR"))).unwrap();

    // The `cargo_bin` function will find the executable for the `server` crate and the `spawn` method will run it in the background.
    // When the `_guard` variable goes out of scope the server process will be killed.
    let _guard = ProcessGuard::new(Command::cargo_bin("server").unwrap().spawn().unwrap());

    // We make a GET request to the /hello path on the server. The `text` method will return the response body as a String.
    let response = get("http://127.0.0.1:8080/hello").unwrap().text().unwrap();

    // The `assert_eq` macro will assert if both parameters are the same.
    // In this case the response body should be the same as the contents of the hello.html file.
    assert_eq!(expected_response, response);

    // The `server` process will be killed when the current stackframe is released.
}
