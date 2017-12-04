#[cfg(test)]
mod test {

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
    fn hello_name_param_is_number() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/1234").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, 1234".into()));
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
