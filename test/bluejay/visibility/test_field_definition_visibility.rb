# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Validation
    class TestFieldDefinitionVisibility < Minitest::Test
      class Visibility < T::Struct
        extend(T::Sig)
        include(Bluejay::Visibility)

        const(:flag, Symbol)

        sig { override.returns(String) }
        def cache_key
          flag.to_s
        end

        sig { override.params(context: T.untyped).returns(T::Boolean) }
        def visible?(context)
          context[:flags].include?(flag)
        end
      end

      class QueryRoot
        include(Base::ObjectType)
        include(Base::QueryRoot)

        class << self
          extend(T::Sig)

          sig { override.returns(ObjectTypeDefinition) }
          def definition
            ObjectTypeDefinition.new(
              name: "QueryRoot",
              field_definitions: [
                FieldDefinition.new(
                  name: "visible",
                  type: OutputType.new(type: Scalar::String, required: true),
                  visibility: Visibility.new(flag: :visible),
                ),
                FieldDefinition.new(
                  name: "invisible",
                  type: OutputType.new(type: Scalar::String, required: true),
                  visibility: Visibility.new(flag: :invisible),
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
        include(Base::Schema)

        class << self
          extend(T::Sig)

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

      def test_field_definition_visibility
        expected_schema_dump = <<~GQL
          type QueryRoot {
            visible: String!
          }

          schema {
            query: QueryRoot
          }
        GQL

        assert_equal(
          expected_schema_dump,
          SchemaDefinition.definition.to_definition({ flags: [:visible, :invisible] }),
        )
      end
    end
  end
end
