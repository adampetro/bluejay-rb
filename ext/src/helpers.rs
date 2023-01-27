mod wrapped_definition;
pub use wrapped_definition::{HasDefinitionWrapper, WrappedDefinition};
mod public_name;
pub use public_name::public_name;
mod wrapped_struct;
pub use wrapped_struct::{WrappedStruct, WrappedStructMap};

use magnus::{RArray, TryConvert, Error};

pub fn from_rarray<T: TryConvert>(arr: RArray) -> Result<Vec<T>, Error> {
    arr.each().map(|value| {
        value.and_then(|value| value.try_convert())
    }).collect()
}
