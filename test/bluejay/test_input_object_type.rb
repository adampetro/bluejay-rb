# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestInputObjectType < Minitest::Test
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

    class MyInputObjectType < InputObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def input_field_definitions
          [
            InputValueDefinition.new(name: "myArg", type: lit!(it!(Scalar::String)), description: "This is my arg"),
            InputValueDefinition.new(name: "mySelf", type: it(MyInputObjectType)),
            InputValueDefinition.new(name: "myEnum", type: it(MyEnumType)),
          ]
        end
      end
    end

    def test_coerce_input_valid
      result = MyInputObjectType.coerce_input({ "myArg" => ["X"], "mySelf" => { "myArg" => "Y" }, "myEnum" => "ONE" })

      assert_predicate(result, :ok?)
      assert_equal(
        MyInputObjectType.new(
          my_arg: ["X"],
          my_self: MyInputObjectType.new(my_arg: ["Y"], my_self: nil, my_enum: nil),
          my_enum: "ONE",
        ),
        result.unwrap,
      )
    end

    def test_coerce_input_extraneous_field
      result = MyInputObjectType.coerce_input({ "myArg" => [], "notAField" => nil })

      assert_predicate(result, :err?)
      assert_equal(1, result.unwrap_err.length)
      assert_equal(
        Bluejay::CoercionError.new("No field named `notAField` on MyInputObjectType", []),
        result.unwrap_err.first,
      )
    end

    def test_coerce_input_field_wrong_type
      result = MyInputObjectType.coerce_input({ "myArg" => 1 })

      assert_predicate(result, :err?)
      assert_equal(1, result.unwrap_err.length)
      assert_equal(
        Bluejay::CoercionError.new("No implicit conversion of integer to String", ["myArg"]),
        result.unwrap_err.first,
      )
    end

    def test_coerce_input_nested_error
      result = MyInputObjectType.coerce_input({ "myArg" => [], "mySelf" => { "myArg" => nil } })

      assert_predicate(result, :err?)
      assert_equal(1, result.unwrap_err.length)
      assert_equal(
        Bluejay::CoercionError.new("Got null when a non-null value was expected", ["mySelf", "myArg"]),
        result.unwrap_err.first,
      )
    end

    def test_initialize_and_accessors
      instance = MyInputObjectType.new(
        my_arg: ["X"],
        my_self: MyInputObjectType.new(my_arg: ["Y"], my_self: nil, my_enum: nil),
        my_enum: "ONE",
      )

      assert_equal(["X"], instance.my_arg)
      assert_equal(MyInputObjectType.new(my_arg: ["Y"], my_self: nil, my_enum: nil), instance.my_self)
      assert_equal("ONE", instance.my_enum)
    end

    def test_initialize_freezes
      assert_predicate(MyInputObjectType.new(my_arg: ["X"], my_self: nil, my_enum: nil), :frozen?)
    end
  end
end
