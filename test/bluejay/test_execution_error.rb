# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestExecutionError < Minitest::Test
    def test_it_responds_to_to_h
      err = Bluejay::ExecutionError.new("Something went wrong", ["root", "field", "0", "thing"])
      expected_h = {
        "message" => "Something went wrong",
        "path" => ["root", "field", "0", "thing"],
      }

      assert_equal(expected_h, err.to_h)
    end
  end
end
