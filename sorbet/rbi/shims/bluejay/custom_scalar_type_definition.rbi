# typed: strict
# frozen_string_literal: true

module Bluejay
  class CustomScalarTypeDefinition
    sig do
      params(
        name: String,
        description: T.nilable(String),
        directives: T::Array[Base::Directive],
        specified_by_url: T.nilable(String),
        ruby_class: Base::CustomScalarType::ClassMethods,
        internal_representation_sorbet_type_name: String,
      ).void
    end
    def initialize(name:, description:, directives:, specified_by_url:, ruby_class:,
      internal_representation_sorbet_type_name:)
    end
  end
end
