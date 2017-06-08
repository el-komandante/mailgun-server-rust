extern crate rustc_serialize;
extern crate hyper;
#[macro_use] extern crate nickel;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response};
use hyper::{Client};
use hyper::header::{Headers, Authorization, Basic};

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Email {
    name: String,
    email_address: String,
    message: String
}
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct MailgunMessage {
    // username: String,
    // password: String,
    to: String,
    from: String,
    subject: String,
    message: String,
}
fn main() {
    use std::io::Read;
    use rustc_serialize::json;

    let mut server = Nickel::new();
    let mut router = Nickel::router();
    let client = Client::new();
    router.post("/messages", middleware! { |request, response|
        let api_key = "key-fd498ac21f74f98caa6ce8f08f8f27c1".to_string();
        // let domain_name = "https://api.mailgun.net/v3/sandbox9fe98e320e20473a93c1b15356c63158.mailgun.org/messages";
        let domain_name = "127.0.0.1:6767/test";
        let username = "api".to_string();
        let to = "rudydeberry@hotmail.com".to_string();
        let from = "rudydeberry@sandbox9fe98e320e20473a93c1b15356c63158.mailgun.org".to_string();
        let email = request.json_as::<Email>().unwrap();
        let subject = format!("new message from <{}>", email.email_address).to_string();
        // let req = format!("username=api&password={}&to={}&from={}&subject=Message from {} <{}>&message={}", api_key, to, from, email.name, email.email_address, email.message);
        let mut headers = Headers::new();
        headers.set(
           Authorization(
               Basic {
                   username: username.to_owned(),
                   password: Some(api_key.to_owned())
               }
           )
        );
        let req: MailgunMessage = MailgunMessage  {
            to: to,
            from: from,
            message: email.message,
            subject: subject
        };
        let body = json::encode(&req).unwrap();
        let mut res = client.post(domain_name)
                        // .headers(headers)
                        .body("not super sure about this")
                        .send()
                        .unwrap();
        let mut res_body = String::new();
        res.read_to_string(&mut res_body).unwrap();
        println!("Response: {}", &res_body);
    });
    router.post("/test", middleware! { |request, response|
        println!("{:?}", request);
    });

    server.utilize(router);
    server.listen("127.0.0.1:6767");
}
