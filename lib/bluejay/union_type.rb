# typed: strict
# frozen_string_literal: true

module Bluejay
  class UnionType
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

      sig { abstract.returns(T::Array[UnionMemberType]) }
      def member_types; end

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

      sig { returns(UnionTypeDefinition) }
      def definition
        @definition ||= T.let(
          UnionTypeDefinition.new(name: graphql_name, description:, member_types:, directives:),
          T.nilable(UnionTypeDefinition),
        )
      end
    end
  end
end
