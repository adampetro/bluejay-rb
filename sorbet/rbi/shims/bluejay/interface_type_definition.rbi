# typed: strict
# frozen_string_literal: true

module Bluejay
  class InterfaceTypeDefinition
    sig do
      params(
        name: String,
        field_definitions: T::Array[FieldDefinition],
        interface_implementations: T::Array[InterfaceImplementation],
        description: T.nilable(String),
        directives: T::Array[Base::Directive],
      ).void
    end
    def initialize(name:, field_definitions:, interface_implementations:, description:, directives:); end
  end
end
