# typed: strict

module Bluejay
  class InputValueDefinition
    sig { params(name: String, type: InputTypeReference, description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, type:, description: nil, directives: []); end

    sig { returns(String) }
    def name; end

    sig { returns(InputTypeReference) }
    def type; end

    sig { returns(String) }
    def ruby_name; end
  end
end
