# typed: strict
# frozen_string_literal: true

module Bluejay
  class ObjectType
    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(OutputTypeShorthands)
      include(InputTypeShorthands)
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

      sig { overridable.returns(T::Array[Directive]) }
      def directives
        []
      end

      private

      sig { params(name: Symbol).returns(T.untyped) }
      def const_missing(name)
        if name == :Interface
          definition
          const_get(:Interface)
        else
          super
        end
      end

      sig { returns(ObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(ObjectTypeDefinition))
        @definition ||= begin
          graphql_name = self.graphql_name
          field_definitions = self.field_definitions
          interface_implementations = self.interface_implementations
          interface = Module.new do |mod|
            field_definitions.each do |field_definition|
              mod.define_method(field_definition.resolver_method_name) { graphql_name }
            end

            interface_implementations.each do |interface_implementation|
              # interface_implementation.interface.send(:definition)
              mod.include(interface_implementation.interface.const_get(:Interface))
            end

            mod.define_method(:resolve_typename) { graphql_name }
          end
          const_set(:Interface, interface)
          ObjectTypeDefinition.new(
            name: graphql_name,
            description:,
            field_definitions:,
            interface_implementations:,
            directives:,
          )
        end
      end
    end
  end
end
