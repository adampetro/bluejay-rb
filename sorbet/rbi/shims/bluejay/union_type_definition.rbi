# typed: strict
# frozen_string_literal: true

module Bluejay
  class UnionTypeDefinition
    sig do
      params(
        name: String,
        member_types: T::Array[UnionMemberType],
        description: T.nilable(String),
        directives: T::Array[Directive],
        field_definitions: T::Array[FieldDefinition],
      ).void
    end
    def initialize(name:, member_types:, description:, directives:, field_definitions:); end
  end
end
