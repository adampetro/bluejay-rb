# typed: ignore
# frozen_string_literal: true

require "bluejay"
require "sorbet-runtime"
require_relative "bench"

module Graph
  class Wrapper < ::Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
      def field_definitions
        [
          ::Bluejay::FieldDefinition.new(name: "value", type: ot!(::Bluejay::Scalar::String)),
        ]
      end
    end
  end

  class QueryRoot < ::Bluejay::QueryRoot
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
      def field_definitions
        [
          ::Bluejay::FieldDefinition.new(name: "wrappers", type: lot!(ot!(Wrapper))),
        ]
      end
    end
  end

  class Schema < ::Bluejay::Schema
    class << self
      extend(T::Sig)

      sig { override.returns(T.class_of(::Bluejay::QueryRoot)) }
      def query
        QueryRoot
      end
    end
  end
end

module Models
  class Wrapper < T::Struct
    extend(T::Sig)
    include(Graph::Wrapper::Interface)
    const(:value, String)
  end

  class QueryRoot < T::Struct
    extend(T::Sig)
    include(Graph::QueryRoot::Interface)

    const(:wrappers, T::Array[Wrapper])
  end

  class SchemaRoot < T::Struct
    extend(T::Sig)
    include(Graph::Schema::Root)

    const(:query, QueryRoot)
  end
end

n = 10
root_value = Models::QueryRoot.new(wrappers: [Models::Wrapper.new(value: "foo")] * n)
schema_root_value = Models::SchemaRoot.new(query: root_value)
query = <<~GQL
  {
    wrappers { value }
  }
GQL

result = Graph::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value)

unless result.errors.empty?
  raise "encountered an error"
end

unless result.value == { "wrappers" => [{ "value" => "foo"}] * n }
  raise "results not equal"
end

puts "Ruby-from-Rust duration (us): #{result.ruby_duration_us}, (ns): #{result.ruby_duration_ns}"

duration = 0
root_value.wrappers.each do |wrapper|
  start = Time.now
  wrapper.value
  duration += (Time.now.to_f - start.to_f) * 1_000_000
end
puts "Ruby duration (us): #{duration}"
