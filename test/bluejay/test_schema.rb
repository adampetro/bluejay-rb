# typed: ignore
# frozen_string_literal: true

require "test_helper"
require "date"

module Bluejay
  class TestSchema < Minitest::Test
    class NameInput < InputType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def input_field_definitions
          [
            InputValueDefinition.new(name: "first", type: it!(Scalar::String)),
            InputValueDefinition.new(name: "last", type: it(Scalar::String)),
          ]
        end
      end
    end

    class DateScalar < CustomScalarType
      extend(T::Generic)

      InternalRepresentation = type_template { { fixed: Date } }

      class << self
        extend(T::Sig)

        sig { override.returns(String) }
        def graphql_name = "Date"

        sig { override.params(value: InternalRepresentation).returns(Result[T.untyped, String]) }
        def coerce_result(value)
          if value == Date.today
            Result.ok(value.iso8601)
          else
            Result.err("Did not return today")
          end
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
              name: "hello",
              type: ot!(Scalar::String),
              argument_definitions: [
                InputValueDefinition.new(name: "name", type: it!(NameInput)),
              ],
            ),
            FieldDefinition.new(
              name: "today",
              type: ot!(DateScalar),
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
      class QueryRoot < T::Struct
        extend(T::Sig)
        include(TestSchema::QueryRoot::Interface)

        const(:today, Date, factory: -> { Date.today })

        sig { params(name: NameInput).returns(String) }
        def resolve_hello(name)
          "Hello, #{name.first} #{name.last}!"
        end

        sig { returns(Date) }
        def resolve_today
          today
        end
      end

      class SchemaRoot < T::Struct
        extend(T::Sig)
        include(MySchema::Root)

        const(:query, QueryRoot, factory: -> { QueryRoot.new })
      end
    end

    def test_foo
      refute_nil(MySchema.send(:definition))
    end

    def test_execute
      query = <<~GQL
        query Hello($name: NameInput!) {
          __typename
          hello(name: $name)
          otherHello: hello(name: { first: "John" last: "Smith" })
          today
        }
      GQL
      root = Domain::SchemaRoot.new

      result = MySchema.execute(
        query:,
        operation_name: nil,
        variables: { "name" => { "first" => "Adam", "last" => "Petro" } },
        initial_value: root,
      )

      assert_empty(result.errors)
      assert_equal(
        {
          "__typename" => "QueryRoot",
          "hello" => "Hello, Adam Petro!",
          "otherHello" => "Hello, John Smith!",\
          "today" => Date.today.iso8601,
        },
        result.value,
      )
    end

    def test_execute_custom_scalar_coerce_result_error
      query = "{ today }"
      root = Domain::SchemaRoot.new(query: Domain::QueryRoot.new(today: Date.today.next_day))

      result = MySchema.execute(
        query:,
        operation_name: nil,
        initial_value: root,
      )

      assert_equal(1, result.errors.length)
      assert_equal(
        ExecutionError.new("Field error"),
        result.errors.first,
      )
    end

    def test_validate_query
      query = <<~GQL
        query Hello($name: NameInput!) {
          __typename
          hello(name: $name)
          otherHello: hello(name: { first: "John" last: "Smith" })
        }
      GQL

      assert_empty(MySchema.validate_query(query:))
    end

    def test_interface_module_exists
      assert_instance_of(Module, MySchema.const_get(:Root))
    end

    def test_const_missing
      assert_raises(NameError) { MySchema.const_get(:Foo) }
    end
  end
end
