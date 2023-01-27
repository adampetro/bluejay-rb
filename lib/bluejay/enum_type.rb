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

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig(:final) { returns(EnumTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(EnumTypeDefinition))
        @definition ||= begin
          enum_value_definitions = self.enum_value_definitions
          sorbet_enum = Class.new(T::Enum) do |klass|
            enums do
              enum_value_definitions.each do |evd|
                klass.const_set(evd.name, klass.new)
              end
            end
          end
          const_set(:Type, sorbet_enum)
          EnumTypeDefinition.new(name: graphql_name, enum_value_definitions:, description:, ruby_class: sorbet_enum)
        end
      end
    end
  end
end
