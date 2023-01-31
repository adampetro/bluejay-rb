# typed: true
# frozen_string_literal: true

require_relative "bench"

Bench.all do |x|
  query = File.read("#{__dir__}/large_query.graphql")

  x.report(:bluejay) { raise "parsing failed" unless Bluejay.parse(query) }
  x.report(:graphql) { GraphQL.parse(query) }
  x.compare!
end
