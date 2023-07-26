# typed: strict
# frozen_string_literal: true

module Bluejay
  class EnumTypeDefinition
    sig do
      params(
        name: String,
        enum_value_definitions: T::Array[EnumValueDefinition],
        description: T.nilable(String),
        directives: T::Array[Base::Directive::Instance],
        ruby_class: Base::EnumType,
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, enum_value_definitions:, description:, directives:, ruby_class:, visibility:); end
  end
end
