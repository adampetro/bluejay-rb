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
        city
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

  graphql_test_run = Schemas::GraphQL::Schema.execute(query, root_value:, validate: false)
  bluejay_test_run = Schemas::Bluejay::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value)

  unless graphql_test_run.to_h["errors"].nil? && bluejay_test_run.errors.empty?
    raise "errors returned"
  end

  unless graphql_test_run.to_h["data"] == bluejay_test_run.value
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
