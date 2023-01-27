# typed: strict

module Bluejay
  class CoercionError
    sig { params(message: String, path: T::Array[String]).void }
    def initialize(message, path); end

    sig { returns(String) }
    def message; end

    sig { returns(T::Array[String]) }
    def path; end

    sig { params(other: T.untyped).returns(T::Boolean) }
    def ==(other); end
  end
end
