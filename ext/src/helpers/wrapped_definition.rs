use magnus::{
    exception, gc, memoize, typed_data::Obj, value::Id, Error, IntoValue, Module, Object, RClass,
    RModule, Ruby, TryConvert, TypedData, Value,
};
use once_cell::unsync::OnceCell;

use crate::ruby_api::DirectiveDefinition;

pub trait HasDefinitionWrapper: TypedData {
    /// module that the wrapper must include in its singleton (a.k.a. extend)
    fn required_module() -> RModule;
}

#[derive(Debug, Copy, Clone)]
pub enum Wrapper {
    Class(RClass),
    Module(RModule),
}

impl std::ops::Deref for Wrapper {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Class(class) => class,
            Self::Module(module) => module,
        }
    }
}

impl TryConvert for Wrapper {
    fn try_convert(val: Value) -> Result<Self, Error> {
        if let Some(cls) = RClass::from_value(val) {
            Ok(Self::Class(cls))
        } else if let Some(module) = RModule::from_value(val) {
            Ok(Self::Module(module))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("expected a Module or Class, got {}", val.class()),
            ))
        }
    }
}

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(class) => write!(f, "{}", class),
            Self::Module(module) => write!(f, "{}", module),
        }
    }
}

impl Wrapper {
    pub(crate) fn singleton_class(&self) -> Result<RClass, Error> {
        match self {
            Self::Class(class) => class.singleton_class(),
            Self::Module(module) => module.singleton_class(),
        }
    }
}

#[derive(Debug)]
pub struct WrappedDefinition<T: HasDefinitionWrapper> {
    wrapper: Wrapper,
    memoized_definition: OnceCell<Obj<T>>,
}

impl<T: HasDefinitionWrapper> Clone for WrappedDefinition<T> {
    fn clone(&self) -> Self {
        Self {
            wrapper: self.wrapper,
            memoized_definition: self.memoized_definition.clone(),
        }
    }
}

impl<T: HasDefinitionWrapper> WrappedDefinition<T> {
    pub fn new(wrapper: Wrapper) -> Result<Self, Error> {
        if wrapper
            .singleton_class()?
            .is_inherited(T::required_module())
        {
            Ok(Self {
                wrapper,
                memoized_definition: OnceCell::new(),
            })
        } else {
            Err(Error::new(
                exception::type_error(),
                format!(
                    "module {} singleton does not include {}",
                    wrapper,
                    T::required_module()
                ),
            ))
        }
    }

    fn resolve_definition(&self) -> Result<Obj<T>, Error> {
        self.wrapper
            .funcall(*memoize!(Id: Id::new("definition")), ())
    }

    pub fn get(&self) -> &Obj<T> {
        self.memoized_definition
            .get_or_init(|| self.resolve_definition().unwrap())
    }

    pub fn try_init(&self) -> Result<(), Error> {
        self.memoized_definition
            .get_or_try_init(|| self.resolve_definition())
            .map(|_| ())
    }

    pub fn mark(&self) {
        gc::mark(self.wrapper);
        if let Some(obj) = self.memoized_definition.get() {
            gc::mark(*obj);
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        self.wrapper.funcall("name", ()).unwrap()
    }

    pub fn wrapper(&self) -> Wrapper {
        self.wrapper
    }
}

impl<T: HasDefinitionWrapper> TryConvert for WrappedDefinition<T> {
    fn try_convert(val: Value) -> Result<Self, Error> {
        val.try_convert().and_then(Self::new)
    }
}

impl<T: HasDefinitionWrapper> AsRef<T> for WrappedDefinition<T> {
    fn as_ref(&self) -> &T {
        self.get().get()
    }
}

impl<T: HasDefinitionWrapper> From<&WrappedDefinition<T>> for Value {
    fn from(value: &WrappedDefinition<T>) -> Value {
        **value.get()
    }
}

impl<T: HasDefinitionWrapper> IntoValue for &WrappedDefinition<T> {
    fn into_value_with(self, handle: &Ruby) -> Value {
        (*self.get()).into_value_with(handle)
    }
}

impl<T: HasDefinitionWrapper> PartialEq for WrappedDefinition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.wrapper.eql(other.wrapper).unwrap_or(false)
    }
}

impl WrappedDefinition<DirectiveDefinition> {
    pub(crate) fn class(&self) -> Result<RClass, Error> {
        self.wrapper.try_convert()
    }
}
