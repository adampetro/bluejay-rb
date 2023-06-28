# typed: strict
# frozen_string_literal: true

module Bluejay
  class FieldDefinition
    sig do
      params(
        name: T.any(String, Symbol),
        type: OutputType,
        argument_definitions: T::Array[InputValueDefinition],
        description: T.nilable(String),
        directives: T::Array[Directive],
        resolver_strategy: ResolverStrategy,
        resolver_method_name: T.nilable(Symbol),
        deprecation_reason: T.nilable(String),
      ).void
    end
    def initialize(name:, type:, argument_definitions: [], description: nil, directives: [],
      resolver_strategy: ResolverStrategy::Object, resolver_method_name: nil,
      deprecation_reason: nil)
    end

    sig { returns(String) }
    def name; end

    sig { returns(ResolverStrategy) }
    def resolver_strategy; end

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
