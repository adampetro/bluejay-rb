# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseInputTypeReference = T.type_alias do
    T.any(
      Scalar,
      T.class_of(InputType),
      T.class_of(EnumType),
      T.class_of(CustomScalarType),
    )
  end
end
