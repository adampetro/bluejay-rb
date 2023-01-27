# typed: strict
# frozen_string_literal: true

module Bluejay
  JsonValue = T.type_alias do
    T.any(
      NilClass,
      Integer,
      Float,
      String,
      T::Boolean,
      T::Array[Object],
      T::Hash[String, Object],
    )
  end
end
