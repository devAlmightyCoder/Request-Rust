use error_chain::error_chain;
use std::{error, io::Read};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    println!("res:\n {:?}", res);
    let mut body = String::new();
    // res.read_to_string(&mut body)? is necessary to read the response body 
    // of the HTTP request into a String variable, allowing you to access 
    // and manipulate the content of the response in your code
    res.read_to_string(&mut body)?;

    println!("status: {}", res.status());
    println!("headers:\n{:#?}", res.headers());
    println!("body:\n{}", body);
    Ok(())
}
