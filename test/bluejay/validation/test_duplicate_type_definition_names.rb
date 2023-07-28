# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Validation
    class TestDuplicateTypeDefinitionNames < Minitest::Test
      class Int < Bluejay::CustomScalarType
        extend(T::Generic)

        InternalRepresentation = type_template { { fixed: Object } }

        class << self
          extend(T::Sig)

          sig { override.params(value: T.untyped, context: T.untyped).returns(Result[InternalRepresentation, String]) }
          def coerce_input(value, context)
            raise NotImplementedError
          end
        end
      end

      class QueryRoot < Bluejay::QueryRoot
        class << self
          extend(T::Sig)

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(
                name: "builtinInt",
                type: ot!(Scalar::Int),
              ),
              FieldDefinition.new(
                name: "customInt",
                type: ot!(Int),
              ),
            ]
          end
        end
      end

      def test_duplicate_type_definition_names
        klass = Class.new(Schema) do
          class << self
            def query
              QueryRoot
            end
          end
        end

        e = assert_raises(Errors::NonUniqueDefinitionNameError) do
          klass.send(:definition)
        end

        assert_equal(
          "GraphQL type name `Int` is used in multiple classes: Bluejay::Scalar::Int and "\
            "Bluejay::Validation::TestDuplicateTypeDefinitionNames::Int",
          e.message,
        )
      end
    end
  end
end
