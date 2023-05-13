# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestInputValueDefinition < Minitest::Test
    include(InputTypeShorthands)

    def test_ruby_name_default
      ivd = InputValueDefinition.new(name: "myInputField", type: it!(Scalar::String))

      assert_equal("my_input_field", ivd.ruby_name)
    end

    def test_ruby_name_override
      ivd = InputValueDefinition.new(name: "myInputField", type: it!(Scalar::String), ruby_name: "overridden")

      assert_equal("overridden", ivd.ruby_name)
    end
  end
end
