# typed: strict

module Bluejay
  class ObjectTypeDefinition
    sig { params(name: String, field_definitions: T::Array[FieldDefinition], interface_implementations: T::Array[InterfaceImplementation], description: T.nilable(String)).void }
    def initialize(name:, field_definitions:, interface_implementations:, description:); end

    sig { returns(String) }
    def name; end

    sig { returns(T::Array[FieldDefinition]) }
    def field_definitions; end
  end
end
