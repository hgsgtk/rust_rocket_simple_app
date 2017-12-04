#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> Template {
    let context = TemplateContext {
        name: name
    };
    Template::render("hello", &context)
}

#[get("/hello/<name>/<age>")]
fn hello_name_old(name: String, age: u8) -> String {
    format!("Hello, {}. You are {} years old.", name.as_str(), age)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![hello_world, hello_name, hello_name_old])
        .attach(Template::fairing())
}

fn main() {
   rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;

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
        let response = client.get("/hello/kazuki").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }
    #[test]
    fn hello_name_age() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/kazuki/23").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, kazuki. You are 23 years old.".into()));
    }
    #[test]
    fn hello_name_age_param_is_string() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/hello/kazuki/hogehoge").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
