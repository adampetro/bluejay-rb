# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class Schema < ::Bluejay::Schema
      class << self
        extend(T::Sig)
        include(NameFromClass)
        include(DefinitionGuard)

        sig { override.void }
        def finalize
        end

        sig { override.params(query: T.nilable(T.class_of(QueryRoot))).returns(T.class_of(QueryRoot)) }
        def query(query = nil)
          guard_definition(query)
          @query ||= T.let(nil, T.nilable(T.class_of(QueryRoot)))
          @query = query unless query.nil?
          @query || raise("No query root set")
        end

        sig { override.params(new_description: T.nilable(String)).returns(T.nilable(String)) }
        def description(new_description = nil)
          guard_definition(new_description)
          @description ||= T.let(nil, T.nilable(String))
          @description = new_description unless new_description.nil?
          @description
        end

        sig do
          override(allow_incompatible: true)
            .params(
              query: String,
              root_value: Object,
              operation_name: T.nilable(String),
              variables: T::Hash[String, T.untyped],
            )
            .returns(ExecutionResult)
        end
        def execute(query, root_value: nil, operation_name: nil, variables: {})
          initial_value = const_get(:RootValue).new(query: root_value)
          super(query:, initial_value:, operation_name:, variables:)
        end

        private

        sig { params(name: Symbol).returns(T.untyped) }
        def const_missing(name)
          if name == :RootValue
            root = const_get(:Root)
            container = Class.new(T::Struct) do |cls|
              cls.include(root)
              cls.const(:query, Object)

              # TODO: define mutation getter
            end
            const_set(:RootValue, container)
            container
          else
            super
          end
        end
      end
    end
  end
end
