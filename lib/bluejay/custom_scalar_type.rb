# typed: strict
# frozen_string_literal: true

module Bluejay
  class CustomScalarType
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

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig(:final) { returns(CustomScalarTypeDefinition) }
      def definition
        @definition ||= T.let(
          CustomScalarTypeDefinition.new(name: graphql_name, description:),
          T.nilable(CustomScalarTypeDefinition),
        )
      end
    end
  end
end
