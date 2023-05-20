# typed: strict
# frozen_string_literal: true

module Bluejay
  class ObjectTypeDefinition
    include(Builtin::ObjectTypes::Type::Interface)

    sig do
      params(
        name: String,
        field_definitions: T::Array[FieldDefinition],
        interface_implementations: T::Array[InterfaceImplementation],
        description: T.nilable(String),
        directives: T::Array[Directive],
        ruby_class: T.class_of(ObjectType),
      ).void
    end
    def initialize(name:, field_definitions:, interface_implementations:, description:, directives:, ruby_class:); end

    sig { returns(String) }
    def name; end

    sig { returns(T::Array[FieldDefinition]) }
    def field_definitions; end
  end
end
