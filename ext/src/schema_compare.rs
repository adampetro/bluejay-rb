use magnus::{define_module, function, memoize, Error, Module, RModule};
use bluejay_parser::ast::definition::{DefinitionDocument, SchemaDefinition};

mod compare;
mod changes;
mod diff;
mod helpers;
use diff::Schema;

pub fn root() -> RModule {
    *memoize!(RModule: define_module("Bluejay").unwrap())
}

pub fn schema_compare() -> RModule {
    *memoize!(RModule: root().define_module("SchemaCompare").unwrap())
}

pub fn init() -> Result<(), Error> {
    let module = schema_compare();

    module.define_module_function(
        "compare",
        function!(
            |a: String, b: String| {
                let document_a: DefinitionDocument = DefinitionDocument::parse(&a).unwrap();
                let document_b: DefinitionDocument = DefinitionDocument::parse(&b).unwrap();

                if document_a.definition_count() == 0 || document_b.definition_count() == 0 {
                    return String::from("Invalid schema - no type definitions found");
                }

                let schema_definition_a = SchemaDefinition::try_from(&document_a)
                    .expect("Could not convert document to schema definition");

                let schema_definition_b = SchemaDefinition::try_from(&document_b)
                    .expect("Could not convert document to schema definition");

                let schema = Schema::new(&schema_definition_a, &schema_definition_b);
                let result = compare::Result::new(schema.diff());
                format!("Schema compare successful. Found {} changes.", result.changes().len())
            },
            2
        ),
    )?;

    Ok(())
}
