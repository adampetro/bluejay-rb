# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestSchema < Minitest::Test
    class NameInputObject < InputObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def input_field_definitions
          [
            InputValueDefinition.new(name: "first", type: it!(Scalar::String)),
            InputValueDefinition.new(name: "last", type: it!(Scalar::String)),
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

        sig { override.returns(T.nilable(String)) }
        def specified_by_url
          "https://example.com"
        end

        sig { override.params(value: InternalRepresentation).returns(Result[T.untyped, String]) }
        def coerce_result(value)
          if value == Date.today
            Result.ok(value.iso8601)
          else
            Result.err("Did not return today")
          end
        end

        sig { override.params(value: T.untyped).returns(Result[Date, String]) }
        def coerce_input(value)
          if value.is_a?(String)
            begin
              Result.ok(Date.parse(value))
            rescue Date::Error => e
              Result.err(e.message)
            end
          else
            Result.err("Expected a date encoded as a string")
          end
        end
      end
    end

    class MyEnumType < EnumType
      extend(T::Sig)

      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[EnumValueDefinition]) }
        def enum_value_definitions
          [
            EnumValueDefinition.new(name: "ONE", deprecation_reason: "Testing deprecation"),
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
              name: "hello",
              type: ot!(Scalar::String),
              argument_definitions: [
                InputValueDefinition.new(name: "name", type: it!(NameInputObject)),
              ],
            ),
            FieldDefinition.new(
              name: "today",
              type: ot!(DateScalar),
            ),
            FieldDefinition.new(
              name: "isToday",
              type: ot!(Scalar::Boolean),
              argument_definitions: [
                InputValueDefinition.new(name: "date", type: it!(DateScalar)),
              ],
              resolver_method_name: "today?",
            ),
            FieldDefinition.new(
              name: "deprecatedField",
              type: ot(MyEnumType),
              deprecation_reason: "Testing deprecation",
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
      class QueryRoot < T::Struct
        extend(T::Sig)
        include(TestSchema::QueryRoot::Interface)

        const(:today, Date, factory: -> { Date.today })

        sig { params(name: NameInputObject).returns(String) }
        def hello(name)
          "Hello, #{name.first} #{name.last}!"
        end

        sig { params(date: Date).returns(T::Boolean) }
        def today?(date)
          date == Date.today
        end

        sig { returns(T.nilable(String)) }
        def deprecated_field = nil
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
        query Hello($name: NameInputObject!, $date: Date!) {
          __typename
          hello(name: $name)
          otherHello: hello(name: { first: "John" last: "Smith" })
          today
          isToday(date: $date)
        }
      GQL
      root = Domain::SchemaRoot.new

      result = MySchema.execute(
        query:,
        operation_name: nil,
        variables: { "name" => { "first" => "Adam", "last" => "Petro" }, "date" => Date.today.iso8601 },
        initial_value: root,
      )

      assert_empty(result.errors)
      assert_equal(
        {
          "__typename" => "QueryRoot",
          "hello" => "Hello, Adam Petro!",
          "otherHello" => "Hello, John Smith!",\
          "today" => Date.today.iso8601,
          "isToday" => true,
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
        query Hello($name: NameInputObject!) {
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

    def test_to_definition
      expected = <<~GQL
        scalar Date @specifiedBy(url: "https://example.com")

        enum MyEnumType {
          ONE @deprecated(reason: "Testing deprecation")

          TWO
        }

        input NameInputObject {
          first: String!

          last: String!
        }

        type QueryRoot {
          hello(
            name: NameInputObject!
          ): String!

          today: Date!

          isToday(
            date: Date!
          ): Boolean!

          deprecatedField: MyEnumType @deprecated(reason: "Testing deprecation")
        }

        schema {
          query: QueryRoot
        }
      GQL

      assert_equal(expected, MySchema.to_definition)
    end

    def test_introspection
      query = <<~GQL
        query IntrospectionQuery {
          __schema {
            queryType { name }
            mutationType { name }
            subscriptionType { name }
            types {
              ...FullType
            }
            directives {
              name
              description
              args {
                ...InputValue
              }
              locations
            }
          }
        }

        fragment FullType on __Type {
          kind
          name
          description
          fields(includeDeprecated: true) {
            name
            description
            args {
              ...InputValue
            }
            type {
              ...TypeRef
            }
            isDeprecated
            deprecationReason
          }
          inputFields {
            ...InputValue
          }
          interfaces {
            ...TypeRef
          }
          enumValues(includeDeprecated: true) {
            name
            description
            isDeprecated
            deprecationReason
          }
          possibleTypes {
            ...TypeRef
          }
          specifiedByURL
        }

        fragment InputValue on __InputValue {
          name
          description
          type { ...TypeRef }
          defaultValue
        }

        fragment TypeRef on __Type {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
              }
            }
          }
        }
      GQL
      root = Domain::SchemaRoot.new(query: Domain::QueryRoot.new(today: Date.today))

      result = MySchema.execute(
        query:,
        operation_name: nil,
        initial_value: root,
      )

      assert_empty(result.errors)
      assert_equal(
        JSON.parse(File.read(File.join(File.dirname(__FILE__), "data/introspection.json"))),
        result.value,
      )
    end
  end
end
