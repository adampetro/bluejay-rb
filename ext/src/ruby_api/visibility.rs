use crate::ruby_api::root;
use magnus::{class, exception, gc, memoize, value::Id, Error, Module, RModule, TryConvert, Value};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Visibility {
    cache_key: OnceCell<String>,
    inner: Value,
}

impl Visibility {
    pub(crate) fn mark(&self) {
        gc::mark(&self.inner);
    }

    pub(crate) fn is_visible(&self, context: Value) -> Result<bool, Error> {
        self.inner
            .funcall::<_, _, Value>(*memoize!(Id: Id::new("visible?")), (context,))
            .and_then(|val| {
                if val.is_kind_of(class::true_class()) {
                    Ok(true)
                } else if val.is_kind_of(class::false_class()) {
                    Ok(false)
                } else {
                    Err(Error::new(
                        exception::type_error(),
                        "expected true or false",
                    ))
                }
            })
    }

    pub(crate) fn cache_key(&self) -> Result<&str, Error> {
        self.cache_key
            .get_or_try_init(|| self.inner.funcall(*memoize!(Id: Id::new("cache_key")), ()))
            .map(|s| s.as_str())
    }
}

fn visibility_interface() -> RModule {
    *memoize!(RModule: root().define_module("Visibility").unwrap())
}

impl TryConvert for Visibility {
    fn try_convert(val: Value) -> Result<Self, Error> {
        if !val.class().is_inherited(visibility_interface()) {
            return Err(Error::new(
                exception::type_error(),
                format!("expected a Visibility, got {}", val.class()),
            ));
        }
        Ok(Self {
            cache_key: OnceCell::new(),
            inner: val,
        })
    }
}

pub trait HasVisibility {
    fn visibility(&self) -> Option<&Visibility>;
}
