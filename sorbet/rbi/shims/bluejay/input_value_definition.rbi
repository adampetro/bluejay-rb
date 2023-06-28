# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputValueDefinition
    sig do
      params(
        name: T.any(String, Symbol),
        type: InputType,
        description: T.nilable(String),
        directives: T::Array[Directive],
        ruby_name: T.nilable(Symbol),
      ).void
    end
    def initialize(name:, type:, description: nil, directives: [], ruby_name: nil); end

    sig { returns(String) }
    def name; end

    sig { returns(InputType) }
    def type; end

    sig { returns(Symbol) }
    def ruby_name; end
  end
end
