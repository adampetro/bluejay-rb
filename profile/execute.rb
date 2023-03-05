# typed: ignore
# frozen_string_literal: true

require_relative "../bench/schemas/bluejay"
require_relative "../bench/schemas/models"

root_value = Schemas::Models::QueryRoot.new(teams: Schemas::Models::Team.all)
schema_root_value = Schemas::Models::SchemaRoot.new(query: root_value)
query = <<~GQL
  {
    teams {
      __typename
      name
      # name1: name
      # name2: name
      # name3: name
      # name4: name
      # name5: name
      city
      # city1: city
      # city2: city
      # city3: city
      # city4: city
      # city5: city
      players {
        __typename
        firstName
        lastName
        age
        draftPosition { __typename round selection year }
      }
    }
  }
GQL

loop do
  Schemas::Bluejay::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value)
end
