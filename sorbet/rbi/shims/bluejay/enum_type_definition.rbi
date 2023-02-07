# typed: strict

module Bluejay
  class EnumTypeDefinition
    sig { params(name: String, enum_value_definitions: T::Array[EnumValueDefinition], description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, enum_value_definitions:, description:, directives:); end
  end
end
