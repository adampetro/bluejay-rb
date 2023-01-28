# typed: ignore
# frozen_string_literal: true

require "test_helper"

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
          include(TestSchema::QueryRoot::Interface)

          sig { params(name: NameInput).returns(String) }
          def graphql_hello(name)
            "Hello, #{name.first} #{name.last}!"
          end
        end
      end

      class SchemaRoot
        class << self
          extend(T::Sig)

          sig { returns(T.class_of(QueryRoot)) }
          def query
            QueryRoot
          end
        end
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
        }
      GQL
      root = Domain::SchemaRoot

      result = MySchema.execute(query:, operation_name: nil, variables: { "name" => { "first" => "Adam", "last" => "Petro" } }, initial_value: root)

      assert_equal({ "__typename" => "QueryRoot", "hello" => "Hello, Adam Petro!", "otherHello" => "Hello, John Smith!" }, result.value)
      assert_empty(result.errors)
      puts result.errors.map(&:inspect)
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
  end
end
