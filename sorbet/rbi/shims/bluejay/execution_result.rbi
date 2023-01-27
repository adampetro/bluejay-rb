# typed: strict

module Bluejay
  class ExecutionResult
    sig { returns(T.nilable(T::Hash[String, T.untyped])) }
    def value; end

    sig { returns(T::Array[ExecutionError]) }
    def errors; end
  end
end
