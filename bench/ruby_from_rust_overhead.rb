# typed: ignore
# frozen_string_literal: true

require "bluejay"
require "sorbet-runtime"
require_relative "bench"

def ruby_from_ruby(value, method, args)
  value.send(method, *args)
end

Bench.all do |x|
  query = File.read("#{__dir__}/large_query.graphql")

  raise "aahh!!" unless Bluejay.ruby_from_rust(1, :+, [1]) == 2
  raise "aahh!!" unless ruby_from_ruby(1, :+, [1]) == 2

  x.report(:rust) { Bluejay.ruby_from_rust(1, :+, [1]) }
  x.report(:ruby) { ruby_from_ruby(1, :+, [1]) }
  x.compare!
end
