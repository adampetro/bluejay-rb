# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseOutputType = T.type_alias do
    T.any(
      Scalar,
      T.class_of(EnumType),
      T.class_of(Base::ObjectType),
      T.class_of(UnionType),
      T.class_of(InterfaceType),
      T.class_of(CustomScalarType),
    )
  end
end
