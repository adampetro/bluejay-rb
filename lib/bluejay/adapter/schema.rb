# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class Schema < Base::Schema
      class RootValue < T::Struct
        const(:query, Object)
      end
      private_constant(:RootValue)

      class << self
        extend(T::Sig)
        include(NameFromClass)
        include(DefinitionGuard)

        sig { params(query: T.nilable(T.class_of(QueryRoot))).returns(T.class_of(QueryRoot)) }
        def query(query = nil)
          guard_definition(query)
          @query ||= T.let(nil, T.nilable(T.class_of(QueryRoot)))
          @query = query unless query.nil?
          @query || raise("No query root set")
        end

        sig { params(new_description: T.nilable(String)).returns(T.nilable(String)) }
        def description(new_description = nil)
          guard_definition(new_description)
          @description ||= T.let(nil, T.nilable(String))
          @description = new_description unless new_description.nil?
          @description
        end

        sig do
          params(
            query: String,
            root_value: Object,
            operation_name: T.nilable(String),
            variables: T::Hash[String, T.untyped],
          ).returns(ExecutionResult)
        end
        def execute(query, root_value: nil, operation_name: nil, variables: {})
          initial_value = RootValue.new(query: root_value)
          definition.execute(query, operation_name, variables, initial_value)
        end

        sig { returns(String) }
        def to_definition
          definition.to_definition
        end

        private

        sig { override.returns(SchemaDefinition) }
        def definition
          @definition ||= T.let(nil, T.nilable(SchemaDefinition))
          @definition ||= SchemaDefinition.new(description:, query:, mutation: nil, directives: [], ruby_class: self)
        end
      end
    end
  end
end
