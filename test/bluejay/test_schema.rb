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

    class FooObjectType < ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[FieldDefinition]) }
        def field_definitions
          [
            FieldDefinition.new(name: "foo", type: ot!(Scalar::String)),
          ]
        end
      end
    end

    class BarObjectType < ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[FieldDefinition]) }
        def field_definitions
          [
            FieldDefinition.new(name: "bar", type: ot!(Scalar::String)),
          ]
        end
      end
    end

    class FooOrBarUnionType < UnionType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[UnionMemberType]) }
        def member_types
          [
            UnionMemberType.new(type: FooObjectType),
            UnionMemberType.new(type: BarObjectType),
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
              resolver_method_name: :today?,
            ),
            FieldDefinition.new(
              name: "deprecatedField",
              type: ot(MyEnumType),
              deprecation_reason: "Testing deprecation",
            ),
            FieldDefinition.new(
              name: "fooOrBar",
              type: ot!(FooOrBarUnionType),
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
      class Foo
        class << self
          extend(T::Sig)
          include(FooObjectType::Interface)

          sig { returns(String) }
          def foo = "foo"
        end
      end

      class Bar
        class << self
          extend(T::Sig)
          include(BarObjectType::Interface)

          sig { returns(String) }
          def bar = "bar"
        end
      end

      class QueryRoot < T::Struct
        extend(T::Sig)
        include(TestSchema::QueryRoot::Interface)

        const(:today, Date, factory: -> { Date.today })

        sig { params(name: NameInputObject).returns(String) }
        def hello(name:)
          "Hello, #{name.first} #{name.last}!"
        end

        sig { params(date: Date).returns(T::Boolean) }
        def today?(date:)
          date == Date.today
        end

        sig { returns(T.nilable(String)) }
        def deprecated_field = nil

        sig { returns(T.any(T.class_of(Foo), T.class_of(Bar))) }
        def foo_or_bar = Foo
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
          fooOrBar {
            __typename
            ...on FooObjectType { foo }
            ...on BarObjectType { bar }
          }
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
          "fooOrBar" => {
            "__typename" => "FooObjectType",
            "foo" => "foo",
          },
        },
        result.value,
      )
    end

    def test_execute_preserves_query_order
      query = <<~GQL
        {
          otherHello: hello(name: { first: "John" last: "Smith" })
          hello(name: { first: "John" last: "Smith" })
        }
      GQL
      root = Domain::SchemaRoot.new

      result = MySchema.execute(query:, initial_value: root)

      assert_empty(result.errors)
      assert_equal(
        '{"otherHello":"Hello, John Smith!","hello":"Hello, John Smith!"}',
        result.value.to_json,
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
          fooOrBar {
            __typename
            ...on FooObjectType { foo }
            ...on BarObjectType { bar }
          }
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
        type BarObjectType {
          bar: String!
        }

        scalar Date @specifiedBy(url: "https://example.com")

        type FooObjectType {
          foo: String!
        }

        union FooOrBarUnionType = FooObjectType | BarObjectType

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

          fooOrBar: FooOrBarUnionType!
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
            __typename
            queryType { name }
            mutationType { name }
            subscriptionType { name }
            types {
              ...FullType
            }
            directives {
              __typename
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
          __typename
          kind
          name
          description
          fields(includeDeprecated: true) {
            __typename
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
          __typename
          name
          description
          type { ...TypeRef }
          defaultValue
        }

        fragment TypeRef on __Type {
          __typename
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
