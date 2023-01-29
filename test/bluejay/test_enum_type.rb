# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestEnumType < Minitest::Test
    class MyEnumType < EnumType
      extend(T::Sig)

      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[EnumValueDefinition]) }
        def enum_value_definitions
          [
            EnumValueDefinition.new(name: "ONE"),
            EnumValueDefinition.new(name: "TWO"),
          ]
        end
      end
    end
  end
end
