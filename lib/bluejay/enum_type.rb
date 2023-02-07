# typed: strict
# frozen_string_literal: true

module Bluejay
  class EnumType
    extend(Finalize)

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

      sig { abstract.returns(T::Array[EnumValueDefinition]) }
      def enum_value_definitions; end

      sig { overridable.returns(T::Array[Directive]) }
      def directives
        []
      end

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig(:final) { returns(EnumTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(EnumTypeDefinition))
        @definition ||= EnumTypeDefinition.new(name: graphql_name, enum_value_definitions:, description:, directives:)
      end
    end
  end
end
