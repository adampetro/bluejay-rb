# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Execution
    class TestIncludeAndSkipDirectives < Minitest::Test
      class QueryRoot < Bluejay::QueryRoot
        class << self
          extend(T::Sig)

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(
                name: "foo",
                type: ot!(Scalar::String),
              ),
            ]
          end
        end
      end

      class MySchema < Schema
        class << self
          extend(T::Sig)

          sig { override.returns(T.class_of(Bluejay::QueryRoot)) }
          def query
            QueryRoot
          end
        end
      end

      module Domain
        class QueryRoot
          class << self
            extend(T::Sig)
            include(TestIncludeAndSkipDirectives::QueryRoot::Interface)

            sig { returns(String) }
            def foo
              "foo"
            end
          end
        end

        class SchemaRoot
          class << self
            extend(T::Sig)
            include(MySchema::Root)

            sig { returns(T.class_of(QueryRoot)) }
            def query = QueryRoot
          end
        end
      end

      # include

      def test_execute_include_hard_coded_true
        query = "{ foo @include(if: true) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_equal(
          { "foo" => "foo" },
          result.value,
        )
      end

      def test_execute_include_hard_coded_false
        query = "{ foo @include(if: false) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      def test_execute_include_variable_true
        query = <<~GQL
          query Query($include: Boolean) {
            foo @include(if: $include)
          }
        GQL
        result = MySchema.execute(
          query:,
          variables: { "include" => true },
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_equal(
          { "foo" => "foo" },
          result.value,
        )
      end

      def test_execute_include_variable_false
        query = <<~GQL
          query Query($include: Boolean) {
            foo @include(if: $include)
          }
        GQL
        result = MySchema.execute(
          query:,
          variables: { "include" => false },
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      # skip

      def test_execute_skip_hard_coded_false
        query = "{ foo @skip(if: false) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_equal(
          { "foo" => "foo" },
          result.value,
        )
      end

      def test_execute_skip_hard_coded_true
        query = "{ foo @skip(if: true) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      def test_execute_skip_variable_false
        query = <<~GQL
          query Query($skip: Boolean) {
            foo @skip(if: $skip)
          }
        GQL
        result = MySchema.execute(
          query:,
          variables: { "skip" => false },
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_equal(
          { "foo" => "foo" },
          result.value,
        )
      end

      def test_execute_skip_variable_true
        query = <<~GQL
          query Query($skip: Boolean) {
            foo @skip(if: $skip)
          }
        GQL
        result = MySchema.execute(
          query:,
          variables: { "skip" => true },
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      # include and skip

      def test_execute_include_true_skip_true
        query = "{ foo @include(if: true) @skip(if: true) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      def test_execute_include_true_skip_false
        query = "{ foo @include(if: true) @skip(if: false) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_equal(
          { "foo" => "foo" },
          result.value,
        )
      end

      def test_execute_include_false_skip_true
        query = "{ foo @include(if: false) @skip(if: true) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end

      def test_execute_include_false_skip_false
        query = "{ foo @include(if: false) @skip(if: false) }"

        result = MySchema.execute(
          query:,
          initial_value: Domain::SchemaRoot,
        )

        assert_empty(result.errors)
        assert_empty(result.value)
      end
    end
  end
end
