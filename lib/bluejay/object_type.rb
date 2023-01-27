# typed: strict
# frozen_string_literal: true

module Bluejay
  class ObjectType
    extend(Finalize)

    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(OutputTypeReferenceShorthands)
      include(InputTypeReferenceShorthands)
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

      sig { abstract.returns(T::Array[FieldDefinition]) }
      def field_definitions; end

      sig { overridable.returns(T::Array[InterfaceImplementation]) }
      def interface_implementations
        []
      end

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig { returns(ObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(ObjectTypeDefinition))
        @definition ||= begin
          graphql_name = self.graphql_name
          interface = Module.new do |mod|
            mod.define_method(:graphql___typename) { graphql_name }
          end
          const_set(:Interface, interface)
          ObjectTypeDefinition.new(name: graphql_name, description:, field_definitions:, interface_implementations:)
        end
      end
    end
  end
end
