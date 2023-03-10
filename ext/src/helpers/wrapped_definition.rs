use magnus::{exception, gc, typed_data::Obj, Error, Module, RClass, TryConvert, TypedData, Value};
use once_cell::unsync::OnceCell;

pub trait HasDefinitionWrapper: TypedData {
    fn wrapping_class() -> RClass;
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
        if cls.is_inherited(T::wrapping_class()) {
            Ok(Self {
                cls,
                memoized_definition: OnceCell::new(),
            })
        } else {
            Err(Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    cls,
                    T::wrapping_class()
                ),
            ))
        }
    }

    pub fn get(&self) -> &Obj<T> {
        self.memoized_definition
            .get_or_init(|| self.cls.funcall("definition", ()).unwrap())
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
        let cls = RClass::from_value(val).ok_or_else(|| {
            Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    unsafe { val.classname() },
                    T::wrapping_class()
                ),
            )
        })?;

        Self::new(cls)
    }
}

impl<T: HasDefinitionWrapper> AsRef<T> for WrappedDefinition<T> {
    fn as_ref(&self) -> &T {
        self.get().get()
    }
}
