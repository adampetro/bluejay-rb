# typed: strict

module Bluejay
  class InterfaceTypeDefinition
    sig { params(name: String, field_definitions: T::Array[FieldDefinition], interface_implementations: T::Array[InterfaceImplementation], description: T.nilable(String)).void }
    def initialize(name:, field_definitions:, interface_implementations:, description:); end
  end
end
