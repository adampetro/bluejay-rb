# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    module InputCoercion
      class TestEnumType < Minitest::Test
        class MyEnum < EnumType
          extend(T::Sig)

          class << self
            extend(T::Sig)

            sig { override.returns(T::Array[EnumValueDefinition]) }
            def enum_value_definitions
              [
                EnumValueDefinition.new(name: "ONE"),
                EnumValueDefinition.new(name: "TWO"),
              ]
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
                  name: "myEnum",
                  type: ot!(MyEnum),
                  argument_definitions: [
                    InputValueDefinition.new(name: "myEnum", type: it!(MyEnum)),
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
              include(Execution::InputCoercion::TestEnumType::QueryRoot::Interface)

              sig { params(my_enum: String).returns(String) }
              def my_enum(my_enum:)
                my_enum
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

        def test_coerce_enum_type_from_variables_valid
          query = <<~GQL
            query Query($myEnum: MyEnum!) {
              myEnum(myEnum: $myEnum)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myEnum" => "ONE" },
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myEnum" => "ONE" },
            result.value,
          )
        end

        def test_coerce_enum_type_from_variables_not_a_member
          query = <<~GQL
            query Query($myEnum: MyEnum!) {
              myEnum(myEnum: $myEnum)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myEnum" => "NOT_A_MEMBER" },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No member `NOT_A_MEMBER` on MyEnum")],
            result.errors,
          )
        end

        def test_coerce_enum_type_from_variables_not_a_string
          query = <<~GQL
            query Query($myEnum: MyEnum!) {
              myEnum(myEnum: $myEnum)
            }
          GQL

          result = MySchema.execute(
            query:,
            variables: { "myEnum" => 1 },
            initial_value: Domain::SchemaRoot,
          )

          assert_equal(
            [ExecutionError.new("No implicit conversion of integer to MyEnum")],
            result.errors,
          )
        end

        def test_coerce_enum_type_from_hard_coded_argument
          query = <<~GQL
            query {
              myEnum(myEnum: ONE)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myEnum" => "ONE" },
            result.value,
          )
        end

        def test_coerce_enum_type_from_variables_using_variable_default
          query = <<~GQL
            query Query($myEnum: MyEnum! = ONE) {
              myEnum(myEnum: $myEnum)
            }
          GQL

          result = MySchema.execute(
            query:,
            initial_value: Domain::SchemaRoot,
          )

          assert_empty(result.errors)
          assert_equal(
            { "myEnum" => "ONE" },
            result.value,
          )
        end
      end
    end
  end
end
