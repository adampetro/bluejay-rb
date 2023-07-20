use crate::ruby_api::{
    errors, root, wrapped_value::ValueInner, CoerceInput, Directives, InputType, WrappedValue,
};
use crate::visibility_scoped::{ScopedInputType, VisibilityCache};
use bluejay_core::Value as CoreValue;
use bluejay_printer::value::DisplayValue;
use bluejay_validator::Path;
use convert_case::{Case, Casing};
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    Class, DataTypeFunctions, Error, ExceptionClass, Module, Object, RArray, RHash, RString,
    Symbol, TypedData, Value,
};
use once_cell::sync::OnceCell;

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InputValueDefinition", mark)]
pub struct InputValueDefinition {
    name: String,
    description: Option<String>,
    r#type: Obj<InputType>,
    directives: Directives,
    default_value: Option<(Value, OnceCell<WrappedValue>)>,
    ruby_name: Symbol,
    name_r_string: RString,
}

impl InputValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, _, ()> = get_kwargs(
            kw,
            &["name", "type"],
            &["description", "directives", "ruby_name", "default_value"],
        )?;
        let (name, r#type): (String, Obj<InputType>) = args.required;
        type OptionalArgs = (
            Option<Option<String>>,
            Option<RArray>,
            Option<String>,
            Option<Option<Value>>,
        );
        let (description, directives, ruby_name, default_value): OptionalArgs = args.optional;
        let description = description.unwrap_or_default();
        let directives = directives.try_into()?;
        let ruby_name = ruby_name.unwrap_or_else(|| name.to_case(Case::Snake));
        let ruby_name = Symbol::new(ruby_name.as_str());
        let name_r_string = RString::new(&name);
        Ok(Self {
            name,
            description,
            r#type,
            directives,
            default_value: default_value.flatten().map(|v| (v, OnceCell::new())),
            ruby_name,
            name_r_string,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn r#type(&self) -> &InputType {
        self.r#type.get()
    }

    pub fn default_value(&self) -> Option<&WrappedValue> {
        self.default_value
            .as_ref()
            .map(|v| v.1.get().expect("Default value not coerced"))
    }

    pub fn is_required(&self) -> bool {
        if self.default_value.is_some() {
            false
        } else {
            self.r#type.get().is_required()
        }
    }

    pub(crate) fn ruby_name(&self) -> Symbol {
        self.ruby_name
    }

    pub(crate) fn name_r_string(&self) -> RString {
        self.name_r_string
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }

    pub fn validate_default_value<'a>(
        &'a self,
        visibility_cache: &'a VisibilityCache<'a>,
    ) -> Result<(), Error> {
        if let Some((raw_value, wrapped_value)) = self.default_value.as_ref() {
            wrapped_value
                .get_or_try_init(|| {
                    let scoped_type = ScopedInputType::new(self.r#type.get(), visibility_cache);
                    let path: Path = Default::default();
                    scoped_type
                        .unwrap()
                        .coerced_ruby_value_to_wrapped_value(*raw_value, path)
                        .and_then(|result| {
                            result.map_err(|coercion_errors| {
                                let arr =
                                    RArray::from_iter(coercion_errors.into_iter().map(Obj::wrap));
                                match default_value_error().new_instance((arr, *raw_value)) {
                                    Ok(exception) => Error::Exception(exception),
                                    Err(error) => error,
                                }
                            })
                        })
                })
                .map(|_| ())
        } else {
            Ok(())
        }
    }
}

impl DataTypeFunctions for InputValueDefinition {
    fn mark(&self) {
        gc::mark(self.r#type);
        self.directives.mark();
        if let Some((raw_value, wrapped_value)) = &self.default_value {
            gc::mark(raw_value);
            wrapped_value.get().map(WrappedValue::mark);
        }
        gc::mark(self.ruby_name);
        gc::mark(self.name_r_string);
    }
}

impl bluejay_core::definition::InputValueDefinition for InputValueDefinition {
    type InputType = InputType;
    type Value = ValueInner;
    type Directives = Directives;

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn r#type(&self) -> &Self::InputType {
        self.r#type.get()
    }

    fn default_value(&self) -> Option<&Self::Value> {
        self.default_value
            .as_ref()
            .map(|v| v.1.get().expect("Default value not coerced").as_ref())
    }

    fn directives(&self) -> Option<&Self::Directives> {
        self.directives.to_option()
    }
}

fn default_value_error() -> ExceptionClass {
    *memoize!(ExceptionClass: errors().define_error("DefaultValueError", Default::default()).unwrap())
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputValueDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InputValueDefinition::new, 1))?;
    class.define_method("name", method!(InputValueDefinition::name_r_string, 0))?;
    class.define_method("type", method!(|ivd: &InputValueDefinition| ivd.r#type, 0))?;
    class.define_method("ruby_name", method!(InputValueDefinition::ruby_name, 0))?;
    class.define_method("description", method!(InputValueDefinition::description, 0))?;
    class.define_method(
        "default_value",
        method!(
            |ivd: &InputValueDefinition| ivd
                .default_value()
                .map(|v| DisplayValue::to_string(&v.as_ref().as_ref())),
            0
        ),
    )?;
    class.define_method(
        "resolve_typename",
        method!(|_: &InputValueDefinition| "__InputValue", 0),
    )?;

    Ok(())
}
