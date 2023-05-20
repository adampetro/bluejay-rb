# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestCustomScalarType < Minitest::Test
        class DateScalar < CustomScalarType
          extend(T::Generic)

          InternalRepresentation = type_template { { fixed: Date } }

          class << self
            extend(T::Sig)

            sig { override.returns(String) }
            def graphql_name = "Date"

            sig { override.params(value: InternalRepresentation).returns(Result[T.untyped, String]) }
            def coerce_result(value)
              Result.ok(value.iso8601)
            end

            sig { override.params(value: T.untyped).returns(Result[Date, String]) }
            def coerce_input(value)
              if value.is_a?(String)
                begin
                  Result.ok(Date.parse(value))
                rescue Date::Error => e
                  Result.err("Unable to coerce to #{graphql_name}: #{e.message}")
                end
              else
                Result.err("Expected a date encoded as a string")
              end
            end
          end
        end

        class QueryRoot < Bluejay::QueryRoot
          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[FieldDefinition]) }
            def field_definitions
              [
                FieldDefinition.new(
                  name: "myDate",
                  type: ot!(DateScalar),
                  argument_definitions: [
                    InputValueDefinition.new(name: "myDate", type: it!(DateScalar)),
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
              include(Execution::InputCoercion::TestCustomScalarType::QueryRoot::Interface)

              sig { params(my_date: Date).returns(Date) }
              def my_date(my_date)
                my_date
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

        def test_coerce_custom_scalar_from_variables_valid
          query = <<~GQL
            query Query($myDate: Date!) {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myDate" => "2023-01-01" },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myDate" => "2023-01-01" },
            result.value,
          )
        end

        def test_coerce_custom_scalar_from_variables_invalid
          query = <<~GQL
            query Query($myDate: Date!) {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myDate" => "not a date" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Unable to coerce to Date: invalid date")],
            result.errors,
          )
        end

        def test_coerce_custom_scalar_from_variables_not_a_string
          query = <<~GQL
            query Query($myDate: Date!) {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myDate" => 12 },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Expected a date encoded as a string")],
            result.errors,
          )
        end

        def test_coerce_custom_scalar_from_hard_coded_argument_valid
          query = <<~GQL
            query {
              myDate(myDate: "2023-01-01")
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myDate" => "2023-01-01" },
            result.value,
          )
        end

        def test_coerce_custom_scalar_from_hard_coded_argument_invalid
          query = <<~GQL
            query {
              myDate(myDate: "not a date")
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Unable to coerce to Date: invalid date")],
            result.errors,
          )
        end

        def test_coerce_custom_scalar_from_hard_coded_argument_not_a_string
          query = <<~GQL
            query {
              myDate(myDate: 12)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Expected a date encoded as a string")],
            result.errors,
          )
        end

        def test_coerce_custom_scalar_from_variables_using_variable_default_valid
          query = <<~GQL
            query Query($myDate: Date! = "2023-01-01") {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myDate" => "2023-01-01" },
            result.value,
          )
        end

        def test_coerce_custom_scalar_from_variables_using_variable_default_invalid
          query = <<~GQL
            query Query($myDate: Date! = "not a date") {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Unable to coerce to Date: invalid date")],
            result.errors,
          )
        end

        def test_coerce_custom_scalar_from_variables_using_variable_default_not_a_string
          query = <<~GQL
            query Query($myDate: Date! = 1) {
              myDate(myDate: $myDate)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("Expected a date encoded as a string")],
            result.errors,
          )
        end
      end
    end
  end
end
