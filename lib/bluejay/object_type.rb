# typed: strict
# frozen_string_literal: true

module Bluejay
  class ObjectType
    include(Base::ObjectType)

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

      sig { override.returns(ObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(ObjectTypeDefinition))
        @definition ||= begin
          graphql_name = self.graphql_name
          field_definitions = self.field_definitions + [Builtin.typename_field_definition]
          interface_implementations = self.interface_implementations
          interface = Module.new do |mod|
            field_definitions.each do |field_definition|
              mod.define_method(field_definition.resolver_method_name) { graphql_name }
            end

            interface_implementations.each do |interface_implementation|
              mod.include(T.cast(interface_implementation.interface, T.class_of(InterfaceType)).const_get(:Interface))
            end
          end
          const_set(:Interface, interface)
          ObjectTypeDefinition.new(
            name: graphql_name,
            description:,
            field_definitions:,
            interface_implementations:,
            directives:,
            ruby_class: self,
            visibility: nil,
          )
        end
      end
    end
  end
end
