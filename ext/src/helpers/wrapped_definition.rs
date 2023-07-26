use magnus::{
    exception, gc, memoize, typed_data::Obj, value::Id, Error, IntoValue, RModule, Ruby,
    TryConvert, TypedData, Value,
};
use once_cell::unsync::OnceCell;
use std::fmt::Display;

pub trait HasDefinitionWrapper: TypedData {
    type Wrapper: IntoValue + Clone + Copy + Display + TryConvert;

    /// module that the wrapper must include in its singleton (a.k.a. extend)
    fn required_module() -> RModule;
}

#[derive(Debug)]
pub struct WrappedDefinition<T: HasDefinitionWrapper> {
    wrapper: T::Wrapper,
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
    pub fn new(wrapper: T::Wrapper) -> Result<Self, Error> {
        if wrapper.into_value().is_kind_of(T::required_module()) {
            Ok(Self {
                wrapper,
                memoized_definition: OnceCell::new(),
            })
        } else {
            Err(Error::new(
                exception::type_error(),
                format!(
                    "no implicit coversion of {} to {}",
                    wrapper,
                    T::required_module()
                ),
            ))
        }
    }

    fn resolve_definition(&self) -> Result<Obj<T>, Error> {
        self.wrapper
            .into_value()
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
        gc::mark(&self.wrapper.into_value());
        if let Some(obj) = self.memoized_definition.get() {
            gc::mark(*obj);
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        self.wrapper.to_string()
    }

    pub fn wrapper(&self) -> T::Wrapper {
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
        self.wrapper
            .into_value()
            .eql(&other.wrapper.into_value())
            .unwrap_or(false)
    }
}
