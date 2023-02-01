use magnus::Error;

mod execution;
mod helpers;
mod ruby_api;

#[magnus::init]
fn init() -> Result<(), Error> {
    ruby_api::init()
}
