use magnus::{
    exception, gc, memoize, typed_data::Obj, value::Id, Error, IntoValue, Module, RClass, RModule,
    Ruby, TryConvert, TypedData, Value,
};
use once_cell::unsync::OnceCell;

pub trait HasDefinitionWrapper: TypedData {
    fn required_module() -> RModule;
}

#[derive(Debug)]
pub struct WrappedDefinition<T: HasDefinitionWrapper> {
    cls: RClass,
    memoized_definition: OnceCell<Obj<T>>,
}

impl<T: HasDefinitionWrapper> Clone for WrappedDefinition<T> {
    fn clone(&self) -> Self {
        Self {
            cls: self.cls,
            memoized_definition: self.memoized_definition.clone(),
        }
    }
}

impl<T: HasDefinitionWrapper> WrappedDefinition<T> {
    pub fn new(cls: RClass) -> Result<Self, Error> {
        if cls.is_inherited(T::required_module()) {
            Ok(Self {
                cls,
                memoized_definition: OnceCell::new(),
            })
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("class {} does not include {}", cls, T::required_module()),
            ))
        }
    }

    pub fn get(&self) -> &Obj<T> {
        self.memoized_definition.get_or_init(|| {
            self.cls
                .funcall(*memoize!(Id: Id::new("definition")), ())
                .unwrap()
        })
    }

    pub fn mark(&self) {
        gc::mark(self.cls);
        if let Some(obj) = self.memoized_definition.get() {
            gc::mark(*obj);
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        unsafe { self.cls.name() }.into_owned()
    }

    pub fn class(&self) -> RClass {
        self.cls
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
        self.cls.eql(other.cls).unwrap_or(false)
    }
}
