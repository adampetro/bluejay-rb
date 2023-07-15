# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseOutputType = T.type_alias do
    T.any(
      Scalar,
      Base::EnumType::ClassMethods,
      Base::ObjectType::ClassMethods,
      Base::UnionType::ClassMethods,
      Base::InterfaceType::ClassMethods,
      Base::CustomScalarType::ClassMethods,
    )
  end
end
