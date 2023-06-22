# typed: ignore
# frozen_string_literal: true

require "test_helper"
require "bluejay/adapter"

module Bluejay
  module Adapter
    class TestSchema < Minitest::Test
      class QueryRoot < Adapter::QueryRoot
        field(:my_string, String, description: "This is a string field")
        field(:my_list, [Integer], null: false)
      end

      class MySchema < Schema
        description("This is a schema")

        query(QueryRoot)
      end

      class RootObject < T::Struct
        const(:my_string, String)
        const(:my_list, T::Array[Integer])
      end

      def test_to_definition
        expected = <<~GQL
          type QueryRoot {
            """
            This is a string field
            """
            myString: String

            myList: [Int!]!
          }

          """
          This is a schema
          """
          schema {
            query: QueryRoot
          }
        GQL

        assert_equal(expected, MySchema.to_definition)
      end

      def test_execute
        query = "{ myString myList }"
        root_value = RootObject.new(my_string: "Testing", my_list: [1, 2, 3])

        result = MySchema.execute(query, root_value:)

        assert_empty(result.errors)
        assert_equal(
          { "myString" => "Testing", "myList" => [1, 2, 3] },
          result.value,
        )
      end
    end
  end
end
