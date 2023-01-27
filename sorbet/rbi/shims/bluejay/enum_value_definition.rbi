# typed: strict

module Bluejay
  class EnumValueDefinition
    sig { params(name: String, description: T.nilable(String)).void }
    def initialize(name:, description: nil); end

    sig { returns(String) }
    def name; end
  end
end
