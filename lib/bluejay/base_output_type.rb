# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseOutputType = T.type_alias do
    T.any(
      Scalar,
      Base::EnumType,
      Base::ObjectType,
      Base::UnionType,
      Base::InterfaceType,
      Base::CustomScalarType,
    )
  end
end
