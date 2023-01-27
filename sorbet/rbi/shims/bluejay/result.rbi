# typed: strict

module Bluejay
  class Result
    extend(T::Generic)

    OkType = type_member
    ErrType = type_member

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
