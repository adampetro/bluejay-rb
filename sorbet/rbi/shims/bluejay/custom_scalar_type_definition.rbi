# typed: strict

module Bluejay
  class CustomScalarTypeDefinition
    sig { params(name: String, description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, description:, directives:); end
  end
end
