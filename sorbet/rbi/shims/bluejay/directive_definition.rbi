# typed: strict
# frozen_string_literal: true

module Bluejay
  class DirectiveDefinition
    sig do
      params(
        name: String,
        argument_definitions: T::Array[InputValueDefinition],
        description: T.nilable(String),
        is_repeatable: T::Boolean,
        locations: T::Array[DirectiveLocation],
        ruby_class: Base::Directive::ClassMethods,
      ).void
    end
    def initialize(name:, argument_definitions:, description:, is_repeatable:, locations:, ruby_class:); end

    sig { returns(T::Array[InputValueDefinition]) }
    def argument_definitions; end
  end
end
