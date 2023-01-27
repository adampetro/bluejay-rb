# typed: strict
# frozen_string_literal: true

module Bluejay
  class Schema
    extend(Finalize)

    class << self
      extend(T::Sig)
      extend(T::Helpers)

      abstract!

      sig { overridable.returns(T.nilable(String)) }
      def description
        nil
      end

      sig { abstract.returns(T.class_of(ObjectType)) }
      def query; end

      sig { overridable.returns(T.nilable(T.class_of(ObjectType))) }
      def mutation
        nil
      end

      sig { params(query: String, operation_name: T.nilable(String), variables: T::Hash[String, T.untyped], initial_value: Object).returns(ExecutionResult) }
      def execute(query:, operation_name:, variables: {}, initial_value: nil)
        definition.execute(query, operation_name, variables, initial_value)
      end

      sig { params(query: String).returns(T::Array[ValidationError]) }
      def validate_query(query:)
        definition.validate_query(query)
      end

      protected

      sig(:final) { override.void }
      def finalize
        @definition = T.let(SchemaDefinition.new(description:, query:, mutation:), T.nilable(SchemaDefinition))
      end

      private

      sig { returns(SchemaDefinition) }
      def definition
        T.must(@definition)
      end
    end
  end
end
