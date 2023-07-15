# typed: strict
# frozen_string_literal: true

module Bluejay
  BaseInputType = T.type_alias do
    T.any(
      Scalar,
      Base::InputObjectType::ClassMethods,
      Base::EnumType::ClassMethods,
      Base::CustomScalarType::ClassMethods,
    )
  end
end
