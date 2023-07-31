# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestSchemaCompare < Minitest::Test
    def test_compares_two_sdl_schemas
      old_schema = <<~SCHEMA
        schema {
          query: Query
        }
        input AInput {
          # a
          a: String = "1"
          b: String!
          options: [Options]
        }
        # The Query Root of this schema
        type Query {
          # Just a simple string
          a(anArg: String): String!
          b: BType
          c(arg: Options): Options
        }
        type BType {
          a: String
        }
        type CType {
          a: String @deprecated(reason: "whynot")
          c: Int!
          d(arg: Int): String
        }
        union MyUnion = CType | BType
        interface AnInterface {
          interfaceField: Int!
        }
        interface AnotherInterface {
          anotherInterfaceField: String
        }
        type WithInterfaces implements AnInterface, AnotherInterface {
          a: String!
        }
        type WithArguments {
          a(
            # Meh
            a: Int
            b: String
            option: Options
          ): String
          b(arg: Int = 1): String
        }
        enum Options {
          A
          B
          C
          E
          F @deprecated(reason: "Old")
        }

        # Old
        directive @yolo(
          # Included when true.
          someArg: Boolean!

          anotherArg: String!

          willBeRemoved: Boolean!
        ) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

        type WillBeRemoved {
          a: String
        }

        directive @willBeRemoved on FIELD
      SCHEMA

      new_schema =<<~SCHEMA
        schema {
          query: Query
        }
        input AInput {
          # changed
          a: Int = 1
          c: String!
          options: [Options]
        }
        # Query Root description changed
        type Query {
          # This description has been changed
          a: String!
          b: Int!
          c(arg: Options): Options
        }
        input BType {
          a: String!
        }
        type CType implements AnInterface {
          a(arg: Int): String @deprecated(reason: "cuz")
          b: Int!
          d(arg: Int = 10): String
        }
        type DType {
          b: Int!
        }
        union MyUnion = CType | DType
        interface AnInterface {
          interfaceField: Int!
        }
        interface AnotherInterface {
          b: Int
        }
        type WithInterfaces implements AnInterface {
          a: String!
        }
        type WithArguments {
          a(
            # Description for a
            a: Int
            b: String!
            option: Options
          ): String
          b(arg: Int = 2): String
        }
        enum Options {
          # Stuff
          A
          B
          D
          E @deprecated
          F @deprecated(reason: "New")
        }

        # New
        directive @yolo(
          # someArg does stuff
          someArg: String!

          anotherArg: String! = "Test"
        ) on FIELD | FIELD_DEFINITION

        directive @yolo2(
          # Included when true.
          someArg: String!
        ) on FIELD
      SCHEMA

      puts Bluejay::SchemaCompare.compare(old_schema, new_schema)
    end
  end
end
