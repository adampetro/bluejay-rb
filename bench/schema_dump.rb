# typed: ignore
# frozen_string_literal: true

require_relative "bench"
require_relative "schemas/bluejay"
require_relative "schemas/graphql"

Bench.all do |x|
  x.report(:graphql) do
    Schemas::GraphQL::Schema.to_definition
  end

  x.report(:bluejay) do
    Schemas::Bluejay::Schema.to_definition
  end

  x.compare!
end
