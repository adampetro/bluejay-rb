# typed: ignore
# frozen_string_literal: true

require_relative "bench"
require_relative "schemas/bluejay"
require_relative "schemas/graphql"
require_relative "schemas/models"

Bench.all do |x|
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

  unless Schemas::GraphQL::Schema.validate(query).empty? && Schemas::Bluejay::Schema.validate_query(query:).empty?
    raise "results not equal"
  end

  x.report(:graphql) do
    Schemas::GraphQL::Schema.validate(query)
  end

  x.report(:bluejay) do
    Schemas::Bluejay::Schema.validate_query(query:)
  end

  x.compare!
end
