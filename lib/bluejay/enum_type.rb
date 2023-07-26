# typed: strict
# frozen_string_literal: true

module Bluejay
  class EnumType
    include(Base::EnumType)

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

      private

      sig(:final) { override.returns(EnumTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(EnumTypeDefinition))
        @definition ||= EnumTypeDefinition.new(
          name: graphql_name,
          enum_value_definitions:,
          description:,
          directives:,
          ruby_class: self,
          visibility: nil,
        )
      end
    end
  end
end
