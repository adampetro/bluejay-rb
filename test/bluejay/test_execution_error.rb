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

      err2 = Bluejay::ExecutionError.new(
        "Something else went wrong",
        ["root"],
        [Bluejay::ExecutionError::ErrorLocation.new(10, 12)],
      )
      expected_h2 = {
        "message" => "Something else went wrong",
        "path" => ["root"],
        "locations" => [{ "line" => 10, "column" => 12 }],
      }

      assert_equal(expected_h2, err2.to_h)
    end

    def test_it_can_make_error_locations
      assert(Bluejay::ExecutionError::ErrorLocation.new(5, 10))
    end
  end
end
