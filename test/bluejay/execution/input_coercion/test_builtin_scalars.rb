# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestBuiltinScalars < Minitest::Test
        class QueryRoot < ObjectType
          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[FieldDefinition]) }
            def field_definitions
              [
                FieldDefinition.new(
                  name: "int",
                  type: ot!(Scalar::Int),
                  argument_definitions: [
                    InputValueDefinition.new(name: "int", type: it!(Scalar::Int)),
                  ],
                ),
                FieldDefinition.new(
                  name: "float",
                  type: ot!(Scalar::Float),
                  argument_definitions: [
                    InputValueDefinition.new(name: "float", type: it!(Scalar::Float)),
                  ],
                ),
                FieldDefinition.new(
                  name: "id",
                  type: ot!(Scalar::ID),
                  argument_definitions: [
                    InputValueDefinition.new(name: "id", type: it!(Scalar::ID)),
                  ],
                ),
                FieldDefinition.new(
                  name: "string",
                  type: ot!(Scalar::String),
                  argument_definitions: [
                    InputValueDefinition.new(name: "string", type: it!(Scalar::String)),
                  ],
                ),
                FieldDefinition.new(
                  name: "boolean",
                  type: ot!(Scalar::Boolean),
                  argument_definitions: [
                    InputValueDefinition.new(name: "boolean", type: it!(Scalar::Boolean)),
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
              include(Execution::InputCoercion::TestBuiltinScalars::QueryRoot::Interface)

              sig { params(int: Integer).returns(Integer) }
              def resolve_int(int)
                int
              end

              sig { params(float: Float).returns(Float) }
              def resolve_float(float)
                float
              end

              sig { params(id: T.any(String, Integer)).returns(T.any(String, Integer)) }
              def resolve_id(id)
                id
              end

              sig { params(string: String).returns(String) }
              def resolve_string(string)
                string
              end

              sig { params(boolean: T::Boolean).returns(T::Boolean) }
              def resolve_boolean(boolean)
                boolean
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

        # int

        def test_coerce_int_from_variables_valid
          query = <<~GQL
            query Query($int: Int!) {
              int(int: $int)
            }
          GQL

          [0, 1, (2**31) - 1, -(2**31)].each do |i|
            result = MySchema.execute(
              query:,
              variables: { "int" => i },
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "int" => i },
              result.value,
            )
          end
        end

        def test_coerce_int_from_variables_invalid
          query = <<~GQL
            query Query($int: Int!) {
              int(int: $int)
            }
          GQL

          [2**31, -(2**31) - 1].each do |i|
            result = MySchema.execute(
              query:,
              variables: { "int" => i },
              initial_value: Domain::SchemaRoot,
            )

            assert_equal(
              [ExecutionError.new("Integer values must fit within 32 bits signed")],
              result.errors,
            )
          end
        end

        def test_coerce_int_from_hard_coded_argument_valid
          [0, 1, (2**31) - 1, -(2**31)].each do |i|
            result = MySchema.execute(
              query: "{ int(int: #{i}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "int" => i },
              result.value,
            )
          end
        end

        def test_coerce_int_from_hard_coded_argument_invalid
          [2**31, -(2**31) - 1].each do |i|
            result = MySchema.execute(
              query: "{ int(int: #{i}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_equal(
              [ExecutionError.new("Value too large to fit in a 32-bit signed integer")],
              result.errors,
            )
          end
        end

        def test_coerce_int_from_variables_using_variable_default
          [0, 1, (2**31) - 1, -(2**31)].each do |i|
            query = <<~GQL
              query Query($int: Int! = #{i}) {
                int(int: $int)
              }
            GQL

            result = MySchema.execute(
              query:,
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "int" => i },
              result.value,
            )
          end
        end

        # float

        def test_coerce_float_from_variables_valid
          query = <<~GQL
            query Query($float: Float!) {
              float(float: $float)
            }
          GQL

          [0, 1, 0.0, 1.0, Float::MAX, Float::MIN].each do |f|
            result = MySchema.execute(
              query:,
              variables: { "float" => f },
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "float" => f },
              result.value,
            )
          end
        end

        def test_coerce_float_from_variables_invalid_not_finite
          query = <<~GQL
            query Query($float: Float!) {
              float(float: $float)
            }
          GQL

          [Float::NAN, Float::INFINITY].each do |f|
            result = MySchema.execute(
              query:,
              variables: { "float" => f },
              initial_value: Domain::SchemaRoot,
            )

            assert_equal(
              [ExecutionError.new("Float values must be finite")],
              result.errors,
            )
          end
        end

        def test_coerce_float_from_variables_invalid_wrong_type
          query = <<~GQL
            query Query($float: Float!) {
              float(float: $float)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "float" => "not a float" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of string to Float")],
            result.errors,
          )
        end

        def test_coerce_float_from_hard_coded_argument_valid
          [0, 1, 0.0, 1.0, Float::MAX, Float::MIN].each do |f|
            result = MySchema.execute(
              query: "{ float(float: #{f}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "float" => f },
              result.value,
            )
          end
        end

        def test_coerce_float_from_variables_using_variable_default
          [0, 1, 0.0, 1.0, Float::MAX, Float::MIN].each do |f|
            query = <<~GQL
              query Query($float: Float! = #{f}) {
                float(float: $float)
              }
            GQL

            result = MySchema.execute(
              query:,
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "float" => f },
              result.value,
            )
          end
        end

        # string

        def test_coerce_string_from_variables_valid
          query = <<~GQL
            query Query($string: String!) {
              string(string: $string)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "string" => "this is a string" },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "string" => "this is a string" },
            result.value,
          )
        end

        def test_coerce_string_from_variables_invalid_wrong_type
          query = <<~GQL
            query Query($string: String!) {
              string(string: $string)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "string" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of integer to String")],
            result.errors,
          )
        end

        def test_coerce_string_from_hard_coded_argument_valid
          result = MySchema.execute(
            query: '{ string(string: "this is a string") }',
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "string" => "this is a string" },
            result.value,
          )
        end

        def test_coerce_string_from_variables_using_variable_default
          query = <<~GQL
            query Query($string: String! = "this is a string") {
              string(string: $string)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "string" => "this is a string" },
            result.value,
          )
        end

        # id

        def test_coerce_id_from_variables_valid
          query = <<~GQL
            query Query($id: ID!) {
              id(id: $id)
            }
          GQL

          [1, "my-id"].each do |id|
            result = MySchema.execute(
              query:,
              variables: { "id" => id },
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "id" => id },
              result.value,
            )
          end
        end

        def test_coerce_id_from_variables_invalid_int_too_large
          query = <<~GQL
            query Query($id: ID!) {
              id(id: $id)
            }
          GQL

          [2**31, -(2**31) - 1].each do |id|
            result = MySchema.execute(
              query:,
              variables: { "id" => id },
              initial_value: Domain::SchemaRoot,
            )

            assert_equal(
              [ExecutionError.new("Integer values must fit within 32 bits signed")],
              result.errors,
            )
          end
        end

        def test_coerce_id_from_variables_invalid_wrong_type
          query = <<~GQL
            query Query($id: ID!) {
              id(id: $id)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "id" => 12.3 },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of float to ID")],
            result.errors,
          )
        end

        def test_coerce_id_from_hard_coded_argument_valid
          [1, "my-id"].each do |id|
            result = MySchema.execute(
              query: "{ id(id: #{id.is_a?(String) ? "\"#{id}\"" : id}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "id" => id },
              result.value,
            )
          end
        end

        def test_coerce_id_from_variables_using_variable_default
          [1, "my-id"].each do |id|
            query = <<~GQL
              query Query($id: ID! = #{id.is_a?(String) ? "\"#{id}\"" : id}) {
                id(id: $id)
              }
            GQL

            result = MySchema.execute(
              query:,
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "id" => id },
              result.value,
            )
          end
        end

        # boolean

        def test_coerce_boolean_from_variables_valid
          query = <<~GQL
            query Query($boolean: Boolean!) {
              boolean(boolean: $boolean)
            }
          GQL

          [true, false].each do |b|
            result = MySchema.execute(
              query:,
              variables: { "boolean" => b },
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "boolean" => b },
              result.value,
            )
          end
        end

        def test_coerce_boolean_from_variables_invalid_wrong_type
          query = <<~GQL
            query Query($boolean: Boolean!) {
              boolean(boolean: $boolean)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "boolean" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of integer to Boolean")],
            result.errors,
          )
        end

        def test_coerce_boolean_from_hard_coded_argument_valid
          [true, false].each do |b|
            result = MySchema.execute(
              query: "{ boolean(boolean: #{b}) }",
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "boolean" => b },
              result.value,
            )
          end
        end

        def test_coerce_boolean_from_variables_using_variable_default
          [true, false].each do |b|
            query = <<~GQL
              query Query($boolean: Boolean! = #{b}) {
                boolean(boolean: $boolean)
              }
            GQL

            result = MySchema.execute(
              query:,
              initial_value: Domain::SchemaRoot,
            )

            assert_empty(result.errors)
            assert_equal(
              { "boolean" => b },
              result.value,
            )
          end
        end
      end
    end
  end
end
