# typed: strict
# frozen_string_literal: true

module Bluejay
  class SchemaDefinition
    sig do
      params(
        description: T.nilable(String),
        query: T.class_of(QueryRoot),
        mutation: T.nilable(T.class_of(ObjectType)),
        directives: T::Array[Directive],
        ruby_class: T.class_of(Schema),
      ).void
    end
    def initialize(description:, query:, mutation:, directives:, ruby_class:); end

    sig do
      params(
        query: String,
        operation_name: T.nilable(String),
        variables: T::Hash[String, T.untyped],
        initial_value: Object,
      ).returns(ExecutionResult)
    end
    def execute(query, operation_name, variables, initial_value); end

    sig { params(query: String).returns(T::Array[ValidationError]) }
    def validate_query(query); end

    sig { returns(String) }
    def to_definition; end

    sig do
      params(name: String).returns(T.nilable(T.any(
        ObjectTypeDefinition,
        EnumTypeDefinition,
        InputObjectTypeDefinition,
        UnionTypeDefinition,
        CustomScalarTypeDefinition,
        InterfaceTypeDefinition,
      )))
    end
    def type(name); end
  end
end
