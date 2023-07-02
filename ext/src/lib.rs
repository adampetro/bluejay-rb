use magnus::Error;

mod execution;
mod helpers;
mod ruby_api;
mod visibility_scoped;

#[magnus::init]
fn init() -> Result<(), Error> {
    ruby_api::init()
}
