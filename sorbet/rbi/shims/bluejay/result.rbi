# typed: strict
# frozen_string_literal: true

module Bluejay
  class Result
    extend(T::Generic)

    OkType = type_member
    ErrType = type_member

    class << self
      sig do
        type_parameters(:Ok)
          .params(value: T.type_parameter(:Ok)).returns(Result[T.type_parameter(:Ok), T.untyped])
      end
      def ok(value); end

      sig do
        type_parameters(:Err)
          .params(value: T.type_parameter(:Err)).returns(Result[T.untyped, T.type_parameter(:Err)])
      end
      def err(value); end
    end

    sig { returns(T::Boolean) }
    def ok?; end

    sig { returns(T::Boolean) }
    def err?; end

    sig { returns(ErrType) }
    def unwrap_err; end

    sig { returns(OkType) }
    def unwrap; end
  end
end
