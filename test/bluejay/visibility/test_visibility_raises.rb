# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Validation
    class TestVisibilityRaises < Minitest::Test
      class Visibility
        extend(T::Sig)
        include(Bluejay::Visibility)

        sig { override.returns(String) }
        def cache_key
          "cache_key"
        end

        sig { override.params(context: T.untyped).returns(T::Boolean) }
        def visible?(context)
          raise StandardError, "This should be re-raised"
        end
      end

      class QueryRoot
        class << self
          extend(T::Sig)
          include(Base::ObjectType)
          include(Base::QueryRoot)

          sig { override.returns(ObjectTypeDefinition) }
          def definition
            ObjectTypeDefinition.new(
              name: "QueryRoot",
              field_definitions: [
                FieldDefinition.new(
                  name: "field",
                  type: OutputType.new(type: Scalar::String, required: true),
                  visibility: Visibility.new,
                ),
              ],
              interface_implementations: [],
              description: nil,
              directives: [],
              ruby_class: self,
              visibility: nil,
            )
          end
        end
      end

      class SchemaDefinition
        class << self
          extend(T::Sig)
          include(Base::Schema)

          sig { override.returns(Bluejay::SchemaDefinition) }
          def definition
            Bluejay::SchemaDefinition.new(
              description: nil,
              query: QueryRoot,
              mutation: nil,
              directives: [],
              ruby_class: self,
            )
          end
        end
      end

      def test_visibility_exception_is_re_raised_on_to_definition
        e = assert_raises(StandardError) do
          SchemaDefinition.definition.to_definition(nil)
        end

        assert_equal("This should be re-raised", e.message)
      end

      def test_visibility_exception_is_re_raised_on_validate
        e = assert_raises(StandardError) do
          SchemaDefinition.definition.validate_query("{ field }", nil)
        end

        assert_equal("This should be re-raised", e.message)
      end
    end
  end
end
