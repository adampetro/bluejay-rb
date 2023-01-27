# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestInputType < Minitest::Test
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

    class MyInputType < InputType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def input_field_definitions
          [
            InputValueDefinition.new(name: "myArg", type: lit!(it!(Scalar::String)), description: "This is my arg"),
            InputValueDefinition.new(name: "mySelf", type: it(MyInputType)),
            InputValueDefinition.new(name: "myEnum", type: it(MyEnumType)),
          ]
        end
      end
    end

    def test_coerce_input_valid
      result = MyInputType.coerce_input({ "myArg" => ["X"], "mySelf" => { "myArg" => "Y" }, "myEnum" => "ONE" })
      assert_predicate(result, :ok?)
      assert_equal(MyInputType.new(["X"], MyInputType.new(["Y"], nil, nil), MyEnumType::Type::ONE), result.unwrap)
    end

    def test_coerce_input_extraneous_field
      result = MyInputType.coerce_input({ "myArg" => [], "notAField" => nil, })
      assert_predicate(result, :err?)
      assert_equal(1, result.unwrap_err.length)
      assert_equal(
        Bluejay::CoercionError.new("No field named `notAField` on MyInputType", []),
        result.unwrap_err.first,
      )
    end

    def test_coerce_input_field_wrong_type
      result = MyInputType.coerce_input({ "myArg" => 1 })
      assert_predicate(result, :err?)
      assert_equal(1, result.unwrap_err.length)
      assert_equal(
        Bluejay::CoercionError.new("No implicit conversion of integer to String", ["myArg"]),
        result.unwrap_err.first,
      )
    end
  end
end
