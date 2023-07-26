# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputObjectTypeDefinition
    sig do
      params(
        name: String,
        input_field_definitions: T::Array[InputValueDefinition],
        description: T.nilable(String),
        directives: T::Array[Base::Directive::Instance],
        ruby_class: Base::InputObjectType,
      ).void
    end
    def initialize(name:, input_field_definitions:, description:, directives:, ruby_class:); end

    sig { params(value: T.untyped, context: T.untyped).returns(Result[T.untyped, T::Array[CoercionError]]) }
    def coerce_input(value, context); end

    sig { returns(T::Array[InputValueDefinition]) }
    def input_field_definitions; end
  end
end
