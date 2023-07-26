# typed: strict
# frozen_string_literal: true

module Bluejay
  class QueryRoot < ObjectType
    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(Base::QueryRoot)

      abstract!

      private

      sig { override.returns(ObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(ObjectTypeDefinition))
        @definition ||= begin
          # TODO: reduce duplication with ObjectType.definition
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

            mod.define_method(:resolve_schema) { |schema_class:| schema_class.send(:definition) }
            mod.define_method(:resolve_type) do |name:, schema_class:|
              schema_class.send(:definition).type(name)
            end
          end
          const_set(:Interface, interface)
          introspection_field_definitions = [
            FieldDefinition.new(
              name: "__schema",
              type: ot!(Builtin::ObjectTypes::Schema),
              resolver_method_name: "resolve_schema",
            ),
            FieldDefinition.new(
              name: "__type",
              argument_definitions: [InputValueDefinition.new(name: "name", type: it!(Scalar::String))],
              type: ot(Builtin::ObjectTypes::Type),
              resolver_method_name: "resolve_type",
            ),
          ]
          ObjectTypeDefinition.new(
            name: graphql_name,
            description:,
            field_definitions: field_definitions + introspection_field_definitions,
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
