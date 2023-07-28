use crate::helpers::NewInstanceKw;
use crate::ruby_api::{
    errors, root, wrapped_value::ValueInner, CoerceInput, DirectiveDefinition, Directives,
    HasVisibility, InputType, Visibility, WrappedValue,
};
use crate::visibility_scoped::{ScopedInputType, VisibilityCache};
use bluejay_core::AsIter;
<<<<<<< HEAD
use bluejay_printer::value::ValuePrinter;
=======
>>>>>>> bbe692e (WIP)
use bluejay_validator::Path;
use convert_case::{Case, Casing};
use magnus::QNIL;
use magnus::{
    function, gc, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj, Class,
    DataTypeFunctions, Error, Module, Object, RArray, RHash, RString, Symbol, TypedData, Value,
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
    deprecation_reason: Option<String>,
    visibility: Option<Visibility>,
}

impl InputValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, _, ()> = get_kwargs(
            kw,
            &["name", "type"],
            &[
                "description",
                "directives",
                "ruby_name",
                "default_value",
                "deprecation_reason",
                "visibility",
            ],
        )?;
        let (name, r#type): (String, Obj<InputType>) = args.required;
        type OptionalArgs = (
            Option<Option<String>>,
            Option<RArray>,
            Option<String>,
            Option<Option<Value>>,
            Option<Option<String>>,
            Option<Option<Visibility>>,
        );
        let (description, directives, ruby_name, default_value, deprecation_reason, visibility): OptionalArgs =
            args.optional;
        let description = description.unwrap_or_default();
        let directives = directives.unwrap_or_else(RArray::new);
        let deprecation_reason = deprecation_reason.flatten();
        if let Some(deprecation_reason) = deprecation_reason.as_deref() {
            let directive_definition = DirectiveDefinition::deprecated();
            let args = RHash::from_iter([(
                directive_definition
                    .as_ref()
                    .arguments_definition()
                    .iter()
                    .next()
                    .unwrap()
                    .ruby_name(),
                deprecation_reason,
            )]);
            directives.push(directive_definition.wrapper().new_instance_kw(args)?)?;
        }
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
            deprecation_reason,
            visibility: visibility.flatten(),
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

    pub fn default_value<'a>(
        &'a self,
        visibility_cache: &'a VisibilityCache<'a>,
    ) -> Option<&WrappedValue> {
        if let Some((raw_value, wrapped_value)) = self.default_value.as_ref() {
            match wrapped_value.get_or_try_init(|| {
                let scoped_type = ScopedInputType::new(self.r#type.get(), visibility_cache);
                let path: Path = Default::default();
                scoped_type
                    .unwrap()
                    .coerced_ruby_value_to_wrapped_value(
                        *raw_value,
                        path,
                        visibility_cache.warden().context(),
                    )
                    .and_then(|result| {
                        result.map_err(|coercion_errors| {
                            let arr = RArray::from_iter(coercion_errors.into_iter().map(Obj::wrap));
                            match errors::default_value_error().new_instance((
                                arr,
                                *raw_value,
                                self.name_r_string,
                            )) {
                                Ok(exception) => Error::Exception(exception),
                                Err(error) => error,
                            }
                        })
                    })
            }) {
                Ok(v) => Some(v),
                Err(error) => {
                    visibility_cache.warden().report_error(error);
                    None
                }
            }
        } else {
            None
        }
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
        self.visibility.as_ref().map(Visibility::mark);
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

impl bluejay_visibility::InputValueDefinitionWithVisibility for InputValueDefinition {
    type SchemaDefinition = crate::ruby_api::SchemaDefinition;

    fn default_value<'a>(
        &'a self,
        visibility_cache: &'a bluejay_visibility::Cache<'a, Self::SchemaDefinition>,
    ) -> Option<&Self::Value> {
        self.default_value(visibility_cache).map(|v| v.as_ref())
    }
}

impl HasVisibility for InputValueDefinition {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
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
                .map(|v| ValuePrinter::to_string(v.as_ref())),
            0
        ),
    )?;
    class.define_method(
        "deprecated?",
        method!(
            |ivd: &InputValueDefinition| ivd.deprecation_reason.is_some(),
            0
        ),
    )?;
    class.define_method(
        "deprecation_reason",
        method!(
            |ivd: &InputValueDefinition| ivd.deprecation_reason.as_deref(),
            0
        ),
    )?;
    class.define_method(
        "resolve_typename",
        method!(|_: &InputValueDefinition| "__InputValue", 0),
    )?;

    Ok(())
}
