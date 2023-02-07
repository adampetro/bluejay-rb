# typed: strict

module Bluejay
  class DirectiveDefinition
    sig { params(name: String, argument_definitions: T::Array[InputValueDefinition], description: T.nilable(String), is_repeatable: T::Boolean, ruby_class: T.class_of(Directive)).void }
    def initialize(name:, argument_definitions:, description:, is_repeatable:, ruby_class:); end

    sig { returns(T::Array[InputValueDefinition]) }
    def argument_definitions; end
  end
end
