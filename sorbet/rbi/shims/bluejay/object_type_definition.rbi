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
        directives: T::Array[Base::Directive::Instance],
        ruby_class: Base::ObjectType,
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, field_definitions:, interface_implementations:, description:, directives:, ruby_class:,
      visibility:)
    end

    sig { returns(String) }
    def name; end

    sig { returns(T::Array[FieldDefinition]) }
    def field_definitions; end
  end
end
