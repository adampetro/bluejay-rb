# typed: strict

module Bluejay
  class SchemaDefinition
    sig { params(description: T.nilable(String), query: T.class_of(ObjectType), mutation: T.nilable(T.class_of(ObjectType)), directives: T::Array[Directive]).void }
    def initialize(description:, query:, mutation:, directives:); end

    sig { params(query: String, operation_name: T.nilable(String), variables: T::Hash[String, T.untyped], initial_value: Object).returns(ExecutionResult) }
    def execute(query, operation_name, variables, initial_value); end

    sig { params(query: String).returns(T::Array[ValidationError]) }
    def validate_query(query); end
  end
end
