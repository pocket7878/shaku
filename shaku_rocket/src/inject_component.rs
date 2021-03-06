use std::ops::Deref;

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request, State};

use shaku::{Container, Interface};

/// Used to retrieve a reference to a component from a shaku `Container`.
/// The container should be stored in Rocket's state. Use this struct as a
/// request guard.
///
/// # Example
/// ```rust
/// #![feature(proc_macro_hygiene, decl_macro)]
///
/// #[macro_use] extern crate rocket;
///
/// use shaku::{Component, ContainerBuilder, Interface};
/// use shaku_rocket::Inject;
///
/// trait HelloWorld: Interface {
///     fn greet(&self) -> String;
/// }
///
/// #[derive(Component)]
/// #[shaku(interface = HelloWorld)]
/// struct HelloWorldImpl;
///
/// impl HelloWorld for HelloWorldImpl {
///     fn greet(&self) -> String {
///         "Hello, world!".to_owned()
///     }
/// }
///
/// #[get("/")]
/// fn hello(hello_world: Inject<dyn HelloWorld>) -> String {
///     hello_world.greet()
/// }
///
/// fn main() {
///     let mut builder = ContainerBuilder::new();
///     builder.register_type::<HelloWorldImpl>();
///     let container = builder.build().unwrap();
///
/// # if false { // We don't actually want to launch the server in an example.
///     rocket::ignite()
///         .manage(container)
///         .mount("/", routes![hello])
///         .launch();
/// # }
/// }
/// ```
pub struct Inject<'r, I: Interface + ?Sized>(&'r I);

impl<'a, 'r, I: Interface + ?Sized> FromRequest<'a, 'r> for Inject<'r, I> {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let container: State<'r, Container> = request
            .guard::<State<Container>>()
            .map_failure(|f| (f.0, "Failed to retrieve container from state".to_string()))?;
        let component = container
            .inner()
            .resolve_ref::<I>()
            .map_err(|e| e.to_string())
            .into_outcome(Status::InternalServerError)?;

        Outcome::Success(Inject(component))
    }
}

impl<'r, I: Interface + ?Sized> Deref for Inject<'r, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
