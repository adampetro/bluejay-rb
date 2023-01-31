# typed: ignore
# frozen_string_literal: true

require_relative "bench"
require_relative "schemas/bluejay"
require_relative "schemas/graphql"
require_relative "schemas/models"

Bench.all do |x|
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

  unless Schemas::GraphQL::Schema.execute(query, root_value:, validate: false).to_h["data"] == \
      Schemas::Bluejay::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value).value
    raise "results not equal"
  end

  x.report(:graphql) do
    Schemas::GraphQL::Schema.execute(query, root_value:, validate: false)
  end

  x.report(:bluejay) do
    Schemas::Bluejay::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value)
  end

  x.compare!
end
