# typed: strict

module Bluejay
  class EnumTypeDefinition
    sig { params(name: String, enum_value_definitions: T::Array[EnumValueDefinition], description: T.nilable(String), ruby_class: T.class_of(T::Enum)).void }
    def initialize(name:, enum_value_definitions:, description:, ruby_class:); end
  end
end
