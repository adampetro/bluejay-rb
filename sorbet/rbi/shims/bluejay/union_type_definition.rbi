# typed: strict
# frozen_string_literal: true

module Bluejay
  class UnionTypeDefinition
    sig do
      params(
        name: String,
        member_types: T::Array[UnionMemberType],
        description: T.nilable(String),
        directives: T::Array[Base::Directive::Instance],
        field_definitions: T::Array[FieldDefinition],
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, member_types:, description:, directives:, field_definitions:, visibility:); end
  end
end
