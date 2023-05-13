# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseInputType = T.type_alias do
    T.any(
      Scalar,
      T.class_of(InputObjectType),
      T.class_of(EnumType),
      T.class_of(CustomScalarType),
    )
  end
end
