# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestLists < Minitest::Test
        class QueryRoot < Bluejay::QueryRoot
          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[FieldDefinition]) }
            def field_definitions
              [
                FieldDefinition.new(
                  name: "intList",
                  type: lot!(ot!(Scalar::Int)),
                  argument_definitions: [
                    InputValueDefinition.new(name: "intList", type: lit!(it!(Scalar::Int))),
                  ],
                ),
                FieldDefinition.new(
                  name: "intListList",
                  type: lot!(lot!(ot!(Scalar::Int))),
                  argument_definitions: [
                    InputValueDefinition.new(name: "intListList", type: lit!(lit!(it!(Scalar::Int)))),
                  ],
                ),
              ]
            end
          end
        end

        class MySchema < Schema
          class << self
            extend(T::Sig)

            sig { override.returns(T.class_of(Bluejay::QueryRoot)) }
            def query
              QueryRoot
            end
          end
        end

        module Domain
          class QueryRoot
            class << self
              extend(T::Sig)
              include(Execution::InputCoercion::TestLists::QueryRoot::Interface)

              sig { params(int_list: T::Array[Integer]).returns(T::Array[Integer]) }
              def int_list(int_list)
                int_list
              end

              sig { params(int_list_list: T::Array[T::Array[Integer]]).returns(T::Array[T::Array[Integer]]) }
              def int_list_list(int_list_list)
                int_list_list
              end
            end
          end

          class SchemaRoot
            class << self
              extend(T::Sig)
              include(MySchema::Root)

              sig { returns(T.class_of(QueryRoot)) }
              def query = QueryRoot
            end
          end
        end

        def test_coerce_list_from_variables_valid_list
          query = <<~GQL
            query Query($intList: [Int!]!) {
              intList(intList: $intList)
            }
          GQL

          [[], [1], [1, 2]].each do |l|
            result = MySchema.execute(
              query:,
              variables: { "intList" => l },
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "intList" => l },
              result.value,
            )
          end
        end

        def test_coerce_list_from_variables_valid_inner
          query = <<~GQL
            query Query($intList: [Int!]!) {
              intList(intList: $intList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intList" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intList" => [1] },
            result.value,
          )
        end

        def test_coerce_list_from_variables_multiple_valid_inner
          query = <<~GQL
            query Query($a: Int!, $b: Int!) {
              intList(intList: [$a, $b])
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "a" => 1, "b" => 2 },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intList" => [1, 2] },
            result.value,
          )
        end

        def test_coerce_list_from_variables_invalid
          query = <<~GQL
            query Query($intList: [Int!]!) {
              intList(intList: $intList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intList" => "not an int list" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to integer")],
            result.errors,
          )
        end

        def test_coerce_list_from_variables_invalid_inner
          query = <<~GQL
            query Query($intList: [Int!]!) {
              intList(intList: $intList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intList" => ["not an int"] },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to integer")],
            result.errors,
          )
        end

        def test_coerce_list_from_hard_coded_argument_valid
          [[], [1], [1, 2]].each do |l|
            result = MySchema.execute(
              query: "{ intList(intList: #{l}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "intList" => l },
              result.value,
            )
          end
        end

        def test_coerce_list_from_hard_coded_argument_valid_inner
          result = MySchema.execute(
            query: "{ intList(intList: 1) }",
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intList" => [1] },
            result.value,
          )
        end

        def test_coerce_list_from_hard_coded_argument_invalid
          result = MySchema.execute(
            query: '{ intList(intList: "not an int") }',
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to Int")],
            result.errors,
          )
        end

        def test_coerce_list_from_hard_coded_argument_invalid_inner
          result = MySchema.execute(
            query: '{ intList(intList: ["not an int"]) }',
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to Int")],
            result.errors,
          )
        end

        def test_coerce_list_from_variables_using_variable_default
          [[], [1], [1, 2]].each do |l|
            query = <<~GQL
              query Query($intList: [Int!]! = #{l}) {
                intList(intList: $intList)
              }
            GQL

            result = MySchema.execute(
              query:,
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "intList" => l },
              result.value,
            )
          end
        end

        def test_coerce_list_from_variables_using_variable_default_inner
          query = <<~GQL
            query Query($intList: [Int!]! = 1) {
              intList(intList: $intList)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intList" => [1] },
            result.value,
          )
        end

        def test_coerce_list_from_variables_using_explicit_null
          query = <<~GQL
            query Query($intList: [Int!] = []) {
              intList(intList: $intList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intList" => nil },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Received `null` for $intList, which is invalid for [Int!]!")],
            result.errors,
          )
        end

        def test_coerce_list_of_list_from_variables_valid_list
          query = <<~GQL
            query Query($intListList: [[Int!]!]!) {
              intListList(intListList: $intListList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intListList" => [[1], [2, 3]] },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intListList" => [[1], [2, 3]] },
            result.value,
          )
        end

        def test_coerce_list_of_list_from_variables_valid_inner
          query = <<~GQL
            query Query($intListList: [[Int!]!]!) {
              intListList(intListList: $intListList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intListList" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intListList" => [[1]] },
            result.value,
          )
        end

        def test_coerce_list_of_list_from_variables_invalid_inner
          query = <<~GQL
            query Query($intListList: [[Int!]!]!) {
              intListList(intListList: $intListList)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "intListList" => [1, 2, 3] },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of integer to [Int!]!")] * 3,
            result.errors,
          )
        end

        def test_coerce_list_of_list_from_hard_coded_argument_valid
          result = MySchema.execute(
            query: "{ intListList(intListList: [[1], [2, 3]]) }",
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intListList" => [[1], [2, 3]] },
            result.value,
          )
        end

        def test_coerce_list_of_list_from_hard_coded_argument_valid_inner
          result = MySchema.execute(
            query: "{ intListList(intListList: 1) }",
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "intListList" => [[1]] },
            result.value,
          )
        end

        def test_coerce_list_of_list_from_hard_coded_argument_invalid_inner
          result = MySchema.execute(
            query: "{ intListList(intListList: [1, 2, 3]) }",
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of integer to [Int!]!")] * 3,
            result.errors,
          )
        end
      end
    end
  end
end
