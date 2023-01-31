# typed: true
# frozen_string_literal: true

require "test_helper"

class TestExample < Minitest::Test
  def test_that_it_does_something
    refute_nil(1)
    assert_equal(8, Team.count)
  end

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
          players { firstName lastName }
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
        "players" => [{ "firstName" => "Auston", "lastName" => "Matthews" }],
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
end
