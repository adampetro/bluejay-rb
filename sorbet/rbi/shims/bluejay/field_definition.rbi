# typed: strict
# frozen_string_literal: true

module Bluejay
  class FieldDefinition
    sig do
      params(
        name: String,
        type: OutputType,
        argument_definitions: T::Array[InputValueDefinition],
        description: T.nilable(String),
        directives: T::Array[Directive],
      ).void
    end
    def initialize(name:, type:, argument_definitions: [], description: nil, directives: []); end

    sig { returns(String) }
    def name; end

    sig { returns(String) }
    def resolver_method_name; end

    sig { returns(T::Array[InputValueDefinition]) }
    def argument_definitions; end

    sig { returns(OutputType) }
    def type; end

    sig { returns(T::Array[Directive]) }
    def directives; end
  end
end
