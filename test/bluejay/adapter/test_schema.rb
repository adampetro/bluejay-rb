# typed: ignore
# frozen_string_literal: true

require "test_helper"
require "bluejay/adapter"

module Bluejay
  module Adapter
    class TestSchema < Minitest::Test
      class Nested < Adapter::ObjectType
        field(:my_nested_string, String) do |f|
          f.description("This is a nested string field")
        end

        def my_nested_string = "test"
      end

      class QueryRoot < Adapter::QueryRoot
        field(:my_string, String, description: "This is a string field")
        field(:my_list, [Integer], null: false)
        field(:my_object, Nested)
        field(:add, Integer, null: false) do |f|
          f.argument(:x, Integer, required: true)
          f.argument(:y, Integer, required: true)
        end

        def add(x:, y:)
          x + y
        end
      end

      class MySchema < Schema
        description("This is a schema")

        query(QueryRoot)
      end

      class NestedObject
      end

      class RootObject < T::Struct
        const(:my_string, String)
        const(:my_list, T::Array[Integer])
        const(:my_object, T.nilable(NestedObject))
      end

      def test_to_definition
        expected = <<~GQL
          type Nested {
            """
            This is a nested string field
            """
            myNestedString: String
          }

          type QueryRoot {
            """
            This is a string field
            """
            myString: String

            myList: [Int!]!

            myObject: Nested

            add(
              x: Int!

              y: Int!
            ): Int!
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
        query = "{ myString myList myObject { myNestedString } add(x: 1, y: 2) }"
        root_value = RootObject.new(my_string: "Testing", my_list: [1, 2, 3], my_object: NestedObject.new)

        result = MySchema.execute(query, root_value:)

        assert_empty(result.errors)
        assert_equal(
          { "myString" => "Testing", "myList" => [1, 2, 3], "myObject" => { "myNestedString" => "test" }, "add" => 3 },
          result.value,
        )
      end
    end
  end
end
