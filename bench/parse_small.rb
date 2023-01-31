# typed: true
# frozen_string_literal: true

require_relative "bench"

Bench.all do |x|
  query = <<~GQL
    {
      dog {
        ...fragmentOne
        ...fragmentTwo
      }
    }

    fragment fragmentOne on Dog {
      name
    }

    fragment fragmentTwo on Dog {
      owner {
        name
      }
    }
  GQL

  x.report(:bluejay) { raise "parsing failed" unless Bluejay.parse(query) }
  x.report(:graphql) { GraphQL.parse(query) }
  x.compare!
end
