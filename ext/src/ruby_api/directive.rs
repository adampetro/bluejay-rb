use crate::helpers::{Warden, WrappedDefinition};
use crate::ruby_api::{wrapped_value::ValueInner, CoerceInput, DirectiveDefinition, WrappedValue};
use crate::visibility_scoped::{ScopedDirectiveDefinition, VisibilityCache};
use bluejay_core::{
    definition::prelude::*, Argument as CoreArgument, Arguments as CoreArguments, AsIter,
    Directive as CoreDirective,
};
use magnus::{gc, Error, RObject, TryConvert, Value};

#[derive(Debug)]
pub struct Directive {
    obj: RObject,
    definition: WrappedDefinition<DirectiveDefinition>,
    arguments: Arguments,
}

impl TryConvert for Directive {
    fn try_convert(val: Value) -> Result<Self, Error> {
        let definition: WrappedDefinition<DirectiveDefinition> =
            WrappedDefinition::try_convert(*val.class())?;
        let obj: RObject = val.try_convert()?;
        obj.freeze();
        // TODO: something better than a warden with nil context
        let visibility_cache = VisibilityCache::new(Warden::new(*magnus::QNIL));
        let scoped_directive_definition =
            ScopedDirectiveDefinition::new(definition.as_ref(), &visibility_cache);
        let arguments: Result<Vec<Argument>, Error> = if let Some(arguments_definition) =
            scoped_directive_definition.arguments_definition()
        {
            arguments_definition
                .iter()
                .map(|ivd| -> Result<Argument, Error> {
                    let value: Value = obj.funcall(ivd.inner().ruby_name(), ())?;
                    let value: WrappedValue = ivd
                        .r#type()
                        .coerced_ruby_value_to_wrapped_value(
                            value,
                            Default::default(),
                            *magnus::QNIL,
                        )?
                        .unwrap();
                    let name = ivd.name().to_string();
                    Ok(Argument { name, value })
                })
                .collect()
        } else {
            Ok(Vec::new())
        };
        let arguments = Arguments(arguments?);
        visibility_cache.warden().to_result().map(|_| Self {
            obj,
            definition,
            arguments,
        })
    }
}

impl Directive {
    pub(crate) fn mark(&self) {
        gc::mark(self.obj);
        self.definition.mark();
        self.arguments.0.iter().for_each(|arg| arg.value.mark())
    }

    pub(crate) fn definition(&self) -> &WrappedDefinition<DirectiveDefinition> {
        &self.definition
    }
}

impl CoreDirective<true> for Directive {
    type Arguments = Arguments;

    fn arguments(&self) -> Option<&Self::Arguments> {
        Some(&self.arguments)
    }

    fn name(&self) -> &str {
        self.definition.as_ref().name()
    }
}

#[derive(Debug)]
pub struct Argument {
    name: String,
    value: WrappedValue,
}

impl CoreArgument<true> for Argument {
    type Value = ValueInner;

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn value(&self) -> &Self::Value {
        self.value.as_ref()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Arguments(Vec<Argument>);

impl CoreArguments<true> for Arguments {
    type Argument = Argument;
}

impl AsIter for Arguments {
    type Item = Argument;
    type Iterator<'a> = std::slice::Iter<'a, Self::Item>;

    fn iter(&self) -> Self::Iterator<'_> {
        self.0.iter()
    }
}
