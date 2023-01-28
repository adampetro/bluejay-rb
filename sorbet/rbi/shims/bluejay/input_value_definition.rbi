# typed: strict

module Bluejay
  class InputValueDefinition
    sig { params(name: String, type: InputTypeReference, description: T.nilable(String)).void }
    def initialize(name:, type:, description: nil); end

    sig { returns(String) }
    def name; end

    sig { returns(InputTypeReference) }
    def type; end
  end
end
