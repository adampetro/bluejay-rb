# typed: strict
# frozen_string_literal: true

module Bluejay
  class ExecutionError
    sig { params(message: String, path: T.nilable(T::Array[String])).void }
    def initialize(message, path = nil); end

    sig { returns(String) }
    def message; end

    sig { returns(T.nilable(T::Array[String])) }
    def path; end
  end
end
