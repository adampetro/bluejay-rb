use crate::ruby_api::root;
use bluejay_core::definition::DirectiveLocation as CoreDirectiveLocation;
use bluejay_core::IntoEnumIterator;
use magnus::{Error, Module};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Bluejay::DirectiveLocation")]
#[repr(transparent)]
pub struct DirectiveLocation(CoreDirectiveLocation);

impl From<DirectiveLocation> for CoreDirectiveLocation {
    fn from(val: DirectiveLocation) -> CoreDirectiveLocation {
        val.0
    }
}

impl From<&DirectiveLocation> for CoreDirectiveLocation {
    fn from(val: &DirectiveLocation) -> CoreDirectiveLocation {
        val.0
    }
}

impl From<CoreDirectiveLocation> for DirectiveLocation {
    fn from(value: CoreDirectiveLocation) -> Self {
        Self(value)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("DirectiveLocation", Default::default())?;

    CoreDirectiveLocation::iter().try_for_each(|directive_location| {
        class.const_set(
            directive_location.as_ref(),
            DirectiveLocation::from(directive_location),
        )
    })?;

    Ok(())
}
