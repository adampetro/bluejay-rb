use crate::helpers::WrappedDefinition;
use crate::ruby_api::{value::ValueInner, DirectiveDefinition, Value};
use bluejay_core::{
    Argument as CoreArgument, Arguments as CoreArguments, AsIter, Directive as CoreDirective,
};
use magnus::{gc, Error, RObject, TryConvert, Value as RValue};

#[derive(Debug)]
pub struct Directive {
    obj: RObject,
    definition: WrappedDefinition<DirectiveDefinition>,
    arguments: Arguments,
}

impl TryConvert for Directive {
    fn try_convert(val: RValue) -> Result<Self, Error> {
        let definition: WrappedDefinition<DirectiveDefinition> =
            WrappedDefinition::try_convert(*val.class())?;
        let obj: RObject = val.try_convert()?;
        obj.freeze();
        let arguments: Result<Vec<Argument>, Error> = definition
            .as_ref()
            .arguments_definition()
            .iter()
            .map(|ivd| -> Result<Argument, Error> {
                let value: RValue = obj.funcall(ivd.ruby_name(), ())?;
                let value: Value = value.try_convert()?;
                let name = ivd.name().to_string();
                Ok(Argument { name, value })
            })
            .collect();
        let arguments = Arguments(arguments?);
        Ok(Self {
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
    value: Value,
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
