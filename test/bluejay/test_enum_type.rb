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
            EnumValueDefinition.new(name: "ONE", description: "Description"),
            EnumValueDefinition.new(name: "TWO", description: nil),
          ]
        end
      end
    end

    def test_definition_exists
      refute_nil(MyEnumType.send(:definition))
    end
  end
end
