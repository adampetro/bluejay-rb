# typed: strict
# frozen_string_literal: true

module Bluejay
  class CustomScalarType
    extend(T::Generic)

    InternalRepresentation = type_template

    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(NameFromClass)

      abstract!

      sig { overridable.returns(String) }
      def graphql_name
        name_from_class
      end

      sig { overridable.returns(T.nilable(String)) }
      def description
        nil
      end

      sig { overridable.returns(T::Array[Directive]) }
      def directives
        []
      end

      sig { overridable.params(value: InternalRepresentation).returns(Result[T.untyped, String]) }
      def coerce_result(value)
        Result.ok(value)
      end

      sig { abstract.params(value: T.untyped).returns(Result[InternalRepresentation, String]) }
      def coerce_input(value); end

      sig { overridable.returns(String) }
      def internal_representation_sorbet_type_name
        const_get(:InternalRepresentation).name
      end

      private

      sig(:final) { returns(CustomScalarTypeDefinition) }
      def definition
        @definition ||= T.let(
          CustomScalarTypeDefinition.new(
            name: graphql_name,
            description:,
            directives:,
            ruby_class: self,
            internal_representation_sorbet_type_name:,
          ),
          T.nilable(CustomScalarTypeDefinition),
        )
      end
    end
  end
end
