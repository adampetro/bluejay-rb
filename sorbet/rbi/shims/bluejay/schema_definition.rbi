# typed: strict
# frozen_string_literal: true

module Bluejay
  class SchemaDefinition
    sig do
      params(
        description: T.nilable(String),
        query: Base::QueryRoot::ClassMethods,
        mutation: T.nilable(Base::ObjectType::ClassMethods),
        directives: T::Array[Base::Directive],
        ruby_class: Base::Schema::ClassMethods,
      ).void
    end
    def initialize(description:, query:, mutation:, directives:, ruby_class:); end

    sig do
      params(
        query: String,
        operation_name: T.nilable(String),
        variables: T::Hash[String, T.untyped],
        initial_value: Object,
        context: T.untyped,
      ).returns(ExecutionResult)
    end
    def execute(query, operation_name, variables, initial_value, context); end

    sig { params(query: String, context: T.untyped).returns(T::Array[ValidationError]) }
    def validate_query(query, context); end

    sig { params(context: T.untyped).returns(String) }
    def to_definition(context); end

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
