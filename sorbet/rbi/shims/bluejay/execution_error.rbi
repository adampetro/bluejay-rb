# typed: strict

module Bluejay
  class ExecutionError
    sig { params(message: String).void }
    def initialize(message); end

    sig { returns(String) }
    def message; end
  end
end
