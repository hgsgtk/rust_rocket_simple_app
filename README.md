# First
A Simple API in Rocket (Web framework in rust)

# Hello World
## Trouble Shooting
```
-> % cargo test
   Compiling rocket_api v0.1.0 (file:///Users/user/src/hobby/rust/rocket_api)
error[E0412]: cannot find type `Rocket` in this scope
  --> src/main.rs:11:16
   |
11 | fn rocket() -> Rocket {
   |                ^^^^^^ not found in this scope
   |
help: possible candidate is found in another module, you can import it into scope
   |
1  | use rocket::Rocket;
   |

error: aborting due to previous error

error: Could not compile `rocket_api`.
```
You should change 'Rocket' to 'rocket::Rocket' or add 'use rocket::Rocket'
see detail at https://github.com/SergioBenitez/Rocket/blob/v0.3.3/examples/testing/src/main.rs

If your test is success, cargo output above ...

```
running 1 test
test test::hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

else...

```
running 1 test
test test::hello_world ... FAILED

failures:

---- test::hello_world stdout ----
	🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/hello':
    => GET /hello/hello
GET /hello:
    => Error: No matching routes for GET /hello.
    => Warning: Responding with 404 Not Found catcher.
thread 'test::hello_world' panicked at 'assertion failed: `(left == right)`
  left: `Status { code: 404, reason: "Not Found" }`,
 right: `Status { code: 200, reason: "OK" }`', src/main.rs:32:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test::hello_world

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--bin rocket_api'
```
# Reference
- https://rocket.rs/guide/getting-started/
- https://rocket.rs/guide/testing/
