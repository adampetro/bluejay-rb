# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestInputType < Minitest::Test
        class MyInput < InputType
          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[InputValueDefinition]) }
            def input_field_definitions
              [
                InputValueDefinition.new(name: "myString", type: it!(Scalar::String)),
                InputValueDefinition.new(name: "myInt", type: it!(Scalar::Int)),
              ]
            end
          end
        end

        class QueryRoot < ObjectType
          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[FieldDefinition]) }
            def field_definitions
              [
                FieldDefinition.new(
                  name: "myInput",
                  type: ot!(Scalar::String),
                  argument_definitions: [
                    InputValueDefinition.new(name: "myInput", type: it!(MyInput)),
                  ],
                ),
              ]
            end
          end
        end

        class MySchema < Schema
          class << self
            extend(T::Sig)

            sig { override.returns(T.class_of(ObjectType)) }
            def query
              QueryRoot
            end
          end
        end

        module Domain
          class QueryRoot
            class << self
              extend(T::Sig)
              include(Execution::InputCoercion::TestInputType::QueryRoot::Interface)

              sig { params(my_input: MyInput).returns(String) }
              def resolve_my_input(my_input)
                "myString=`#{my_input.my_string}`, myInt=`#{my_input.my_int}`"
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

        def test_coerce_input_type_from_variables_valid
          query = <<~GQL
            query Query($myInput: MyInput!) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInput" => { "myString" => "A string", "myInt" => 1 } },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInput" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_from_variables_missing_field
          query = <<~GQL
            query Query($myInput: MyInput!) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInput" => { "myString" => "A string" } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No value for required field myInt")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_field_incorrect_type
          query = <<~GQL
            query Query($myInput: MyInput!) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInput" => { "myString" => "A string", "myInt" => "not an int" } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to integer")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_incorrect_type
          query = <<~GQL
            query Query($myInput: MyInput!) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInput" => "not an object" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to MyInput")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_extra_field
          query = <<~GQL
            query Query($myInput: MyInput!) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInput" => { "myString" => "A string", "myInt" => 1, "myExtraField" => {} } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No field named `myExtraField` on MyInput")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_using_variable_default
          query = <<~GQL
            query Query($myInput: MyInput! = { myString: "A string", myInt: 1 }) {
              myInput(myInput: $myInput)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInput" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_from_hard_coded_argument
          query = <<~GQL
            query {
              myInput(myInput: { myString: "A string", myInt: 1 })
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInput" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_with_nested_variables_in_argument
          query = <<~GQL
            query Query($myString: String!, $myInt: Int!) {
              myInput(myInput: { myString: $myString, myInt: $myInt })
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myString" => "A string", "myInt" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInput" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end
      end
    end
  end
end
