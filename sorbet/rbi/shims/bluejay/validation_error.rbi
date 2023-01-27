# typed: strict

module Bluejay
  class ValidationError
    sig { params(message: String).void }
    def initialize(message); end

    sig { returns(String) }
    def message; end
  end
end
