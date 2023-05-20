# typed: true
# frozen_string_literal: true

require "test_helper"

class TestExample < Minitest::Test
  def test_query
    query = <<~GQL
      {
        teams {
          location
          name
        }
      }
    GQL

    result = Graph::Schema.execute(query:, operation_name: nil, initial_value: SchemaRoot)

    assert_empty(result.errors)

    expected_value = {
      "teams" => Team.all.map do |team|
        { "name" => team.name, "location" => team.location }
      end,
    }

    assert_equal(expected_value, result.value)
  end

  def test_query_with_arg
    query = <<~GQL
      query Teams($location: String!) {
        teams(location: $location) {
          location
          name
          players { firstName lastName birthday }
        }
      }
    GQL

    result = Graph::Schema.execute(
      query:,
      operation_name: nil,
      initial_value: SchemaRoot,
      variables: { "location" => "Toronto" },
    )

    assert_empty(result.errors)

    expected_value = {
      "teams" => [{
        "name" => "Maple Leafs",
        "location" => "Toronto",
        "players" => [{ "firstName" => "Auston", "lastName" => "Matthews", "birthday" => "1997-09-17" }],
      }],
    }

    assert_equal(expected_value, result.value)
  end

  def test_query_with_interface
    query = <<~GQL
      query {
        people {
          firstName
          lastName
          __typename
          ...on Player {
            currentTeam { name location }
          }
        }
      }
    GQL

    result = Graph::Schema.execute(query:, operation_name: nil, initial_value: SchemaRoot)

    assert_empty(result.errors)

    expected_value = {
      "people" => [{
        "__typename" => "Player",
        "firstName" => "Auston",
        "lastName" => "Matthews",
        "currentTeam" => { "name" => "Maple Leafs", "location" => "Toronto" },
      }],
    }

    assert_equal(expected_value, result.value)
  end

  def test_schema_dump
    expected = <<~GQL
      scalar Date

      interface Person {
        firstName: String!

        lastName: String!
      }

      type Player implements Person {
        firstName: String!

        lastName: String!

        currentTeam: Team

        birthday: Date!
      }

      type QueryRoot {
        teams(
          location: String
        ): [Team!]!

        people: [Person!]!
      }

      type Team {
        name: String!

        location: String!

        players: [Player!]!
      }

      schema {
        query: QueryRoot
      }
    GQL

    assert_equal(expected, Graph::Schema.to_definition)
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

    result = Graph::Schema.execute(
      query:,
      operation_name: nil,
      initial_value: SchemaRoot,
    )

    assert_empty(result.errors)
    assert_equal(
      JSON.parse(File.read(File.join(File.dirname(__FILE__), "data/introspection.json"))),
      result.value,
    )
  end
end
