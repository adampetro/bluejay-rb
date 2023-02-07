# typed: strict

module Bluejay
  class InputObjectTypeDefinition
    sig { params(name: String, input_field_definitions: T::Array[InputValueDefinition], description: T.nilable(String), directives: T::Array[Directive], ruby_class: T.class_of(InputType)).void }
    def initialize(name:, input_field_definitions:, description:, directives:, ruby_class:); end

    sig { params(value: T.untyped).returns(Result[T.untyped, T::Array[CoercionError]]) }
    def coerce_input(value); end

    sig { returns(T::Array[InputValueDefinition]) }
    def input_field_definitions; end
  end
end
