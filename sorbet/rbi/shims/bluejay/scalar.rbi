# typed: strict

module Bluejay
  class Scalar
    Int = T.let(T.unsafe(nil), Scalar)
    Float = T.let(T.unsafe(nil), Scalar)
    String = T.let(T.unsafe(nil), Scalar)
    Boolean = T.let(T.unsafe(nil), Scalar)
    ID = T.let(T.unsafe(nil), Scalar)
  end
end
