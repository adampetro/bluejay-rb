# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestInputObjectType < Minitest::Test
        class MyInputObject < InputObjectType
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
                  name: "myInputObject",
                  type: ot!(Scalar::String),
                  argument_definitions: [
                    InputValueDefinition.new(name: "myInputObject", type: it!(MyInputObject)),
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
              include(Execution::InputCoercion::TestInputObjectType::QueryRoot::Interface)

              sig { params(my_input_object: MyInputObject).returns(String) }
              def resolve_my_input_object(my_input_object)
                "myString=`#{my_input_object.my_string}`, myInt=`#{my_input_object.my_int}`"
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
            query Query($myInputObject: MyInputObject!) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInputObject" => { "myString" => "A string", "myInt" => 1 } },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInputObject" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_from_variables_missing_field
          query = <<~GQL
            query Query($myInputObject: MyInputObject!) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInputObject" => { "myString" => "A string" } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No value for required field myInt")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_field_incorrect_type
          query = <<~GQL
            query Query($myInputObject: MyInputObject!) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInputObject" => { "myString" => "A string", "myInt" => "not an int" } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to integer")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_incorrect_type
          query = <<~GQL
            query Query($myInputObject: MyInputObject!) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInputObject" => "not an object" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to MyInputObject")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_extra_field
          query = <<~GQL
            query Query($myInputObject: MyInputObject!) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myInputObject" => { "myString" => "A string", "myInt" => 1, "myExtraField" => {} } },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No field named `myExtraField` on MyInputObject")],
            result.errors,
          )
        end

        def test_coerce_input_type_from_variables_using_variable_default
          query = <<~GQL
            query Query($myInputObject: MyInputObject! = { myString: "A string", myInt: 1 }) {
              myInputObject(myInputObject: $myInputObject)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInputObject" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_from_hard_coded_argument
          query = <<~GQL
            query {
              myInputObject(myInputObject: { myString: "A string", myInt: 1 })
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInputObject" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end

        def test_coerce_input_type_with_nested_variables_in_argument
          query = <<~GQL
            query Query($myString: String!, $myInt: Int!) {
              myInputObject(myInputObject: { myString: $myString, myInt: $myInt })
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myString" => "A string", "myInt" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myInputObject" => "myString=`A string`, myInt=`1`" },
            result.value,
          )
        end
      end
    end
  end
end
