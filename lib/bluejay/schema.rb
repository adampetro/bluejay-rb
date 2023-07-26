# typed: strict
# frozen_string_literal: true

module Bluejay
  class Schema
    extend(Finalize)
    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(Base::Schema)

      abstract!

      sig { overridable.returns(T.nilable(String)) }
      def description
        nil
      end

      sig { abstract.returns(T.class_of(QueryRoot)) }
      def query; end

      sig { overridable.returns(T.nilable(T.class_of(ObjectType))) }
      def mutation
        nil
      end

      sig { overridable.returns(T::Array[Directive]) }
      def directives
        []
      end

      sig do
        params(
          query: String,
          initial_value: Object,
          operation_name: T.nilable(String),
          variables: T::Hash[String, T.untyped],
          context: T.untyped,
        ).returns(ExecutionResult)
      end
      def execute(query:, initial_value:, operation_name: nil, variables: {}, context: nil)
        definition.execute(query, operation_name, variables, initial_value, context)
      end

      sig { params(query: String, context: T.untyped).returns(T::Array[ValidationError]) }
      def validate_query(query:, context: nil)
        definition.validate_query(query, context)
      end

      sig { params(context: T.untyped).returns(String) }
      def to_definition(context: nil)
        definition.to_definition(context)
      end

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig { params(name: Symbol).returns(T.untyped) }
      def const_missing(name)
        if name == :Root
          definition
          const_get(:Root)
        else
          super
        end
      end

      sig { override.returns(SchemaDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(SchemaDefinition))
        @definition ||= begin
          mutation = self.mutation
          interface = Module.new do |mod|
            mod.define_method(:query) {}
            if mutation
              mod.define_method(:mutation) {}
            end
          end
          const_set(:Root, interface)
          SchemaDefinition.new(description:, query:, mutation:, directives:, ruby_class: self)
        end
      end
    end
  end
end
