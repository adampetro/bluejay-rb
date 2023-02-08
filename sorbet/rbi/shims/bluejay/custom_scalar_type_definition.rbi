# typed: strict

module Bluejay
  class CustomScalarTypeDefinition
    sig { params(name: String, description: T.nilable(String), directives: T::Array[Directive], ruby_class: T.class_of(CustomScalarType), internal_representation_sorbet_type_name: String).void }
    def initialize(name:, description:, directives:, ruby_class:, internal_representation_sorbet_type_name:); end
  end
end
