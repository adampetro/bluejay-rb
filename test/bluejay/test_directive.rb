# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestDirective < Minitest::Test
    class MyDirective < Directive
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def argument_definitions
          [
            InputValueDefinition.new(name: "myArg", type: it!(Scalar::String), description: "This is my arg"),
          ]
        end

        sig { override.returns(T::Array[DirectiveLocation]) }
        def locations
          [DirectiveLocation::FIELD_DEFINITION]
        end
      end
    end

    def test_initialize_and_accessors
      instance = MyDirective.new("test")

      assert_equal("test", instance.my_arg)
    end

    def test_initialize_freezes
      assert_predicate(MyDirective.new("test"), :frozen?)
    end
  end
end
