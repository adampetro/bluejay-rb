use crate::ruby_api::root;
use magnus::{method, typed_data::IsEql, Error, Module};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ResolverStrategy")]
pub enum ResolverStrategy {
    Object,
    DefinitionInstance,
    DefinitionClass,
}

impl Default for ResolverStrategy {
    fn default() -> Self {
        Self::Object
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ResolverStrategy", Default::default())?;

    class.const_set("Object", ResolverStrategy::Object)?;
    class.const_set("DefinitionInstance", ResolverStrategy::DefinitionInstance)?;
    class.const_set("DefinitionClass", ResolverStrategy::DefinitionClass)?;
    class.define_method("==", method!(<ResolverStrategy as IsEql>::is_eql, 1))?;

    Ok(())
}
