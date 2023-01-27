# typed: strict

module Bluejay
  class CustomScalarTypeDefinition
    sig { params(name: String, description: T.nilable(String)).void }
    def initialize(name:, description:); end
  end
end
