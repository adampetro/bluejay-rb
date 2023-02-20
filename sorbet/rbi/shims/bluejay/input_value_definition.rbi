# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputValueDefinition
    sig do
      params(
        name: String,
        type: InputTypeReference,
        description: T.nilable(String),
        directives: T::Array[Directive],
        ruby_name: T.nilable(String),
      ).void
    end
    def initialize(name:, type:, description: nil, directives: [], ruby_name: nil); end

    sig { returns(String) }
    def name; end

    sig { returns(InputTypeReference) }
    def type; end

    sig { returns(String) }
    def ruby_name; end
  end
end
