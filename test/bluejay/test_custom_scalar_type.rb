# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestCustomScalarType < Minitest::Test
    class MyCustomScalarType < CustomScalarType
    end

    def test_foo
      refute_nil(MyCustomScalarType.send(:definition))
    end
  end
end
