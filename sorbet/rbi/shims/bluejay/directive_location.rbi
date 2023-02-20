# typed: strict
# frozen_string_literal: true

module Bluejay
  class DirectiveLocation
    QUERY = T.let(T.unsafe(nil), DirectiveLocation)
    MUTATION = T.let(T.unsafe(nil), DirectiveLocation)
    SUBSCRIPTION = T.let(T.unsafe(nil), DirectiveLocation)
    FIELD = T.let(T.unsafe(nil), DirectiveLocation)
    FRAGMENT_DEFINITION = T.let(T.unsafe(nil), DirectiveLocation)
    FRAGMENT_SPREAD = T.let(T.unsafe(nil), DirectiveLocation)
    INLINE_FRAGMENT = T.let(T.unsafe(nil), DirectiveLocation)
    VARIABLE_DEFINITION = T.let(T.unsafe(nil), DirectiveLocation)
    SCHEMA = T.let(T.unsafe(nil), DirectiveLocation)
    SCALAR = T.let(T.unsafe(nil), DirectiveLocation)
    OBJECT = T.let(T.unsafe(nil), DirectiveLocation)
    FIELD_DEFINITION = T.let(T.unsafe(nil), DirectiveLocation)
    ARGUMENT_DEFINITION = T.let(T.unsafe(nil), DirectiveLocation)
    INTERFACE = T.let(T.unsafe(nil), DirectiveLocation)
    UNION = T.let(T.unsafe(nil), DirectiveLocation)
    ENUM = T.let(T.unsafe(nil), DirectiveLocation)
    ENUM_VALUE = T.let(T.unsafe(nil), DirectiveLocation)
    INPUT_OBJECT = T.let(T.unsafe(nil), DirectiveLocation)
    INPUT_FIELD_DEFINITION = T.let(T.unsafe(nil), DirectiveLocation)
  end
end
