# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestSchemaCompare < Minitest::Test
    def test_compares_two_sdl_schemas
      old_schema_sdl = <<~SCHEMA
        schema {
          query: Query
        }
        """
        Query root type
        """
        type Query {
          a: String!
        }
        type Foo {
          name: String!
        }
        type Bar {
          name: String!
        }
      SCHEMA

      new_schema_sdl = <<~SCHEMA
        schema {
          query: Query
        }

        """
        New query description
        """
        type Query {
          a: Int
          b: Int!
        }

        type Foo {
          name: String!
          id: ID!
        }

        type NewType {
          name: String!
        }
      SCHEMA

      puts Bluejay::SchemaCompare.compare(old_schema_sdl, new_schema_sdl)
    end
  end
end
