#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello, {}", name.as_str())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![hello_world, hello_name])
}

fn main() {
   rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn hello_name() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/kazuki").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, kazuki".into()));
    }
    #[test]
    fn hello_name_number() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/1234").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, 1234".into()));
    }
}
