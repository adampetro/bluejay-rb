# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Validation
    class TestWrappedDefinitionException < Minitest::Test
      class MyScalar
        include(Bluejay::Base::CustomScalarType)

        class << self
          extend(T::Sig)

          sig { override.returns(Bluejay::CustomScalarTypeDefinition) }
          def definition
            raise StandardError, "This is a test"
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
                name: "myScalar",
                type: ot!(MyScalar),
              ),
            ]
          end
        end
      end

      def test_wrapped_definition_exception
        klass = Class.new(Schema) do
          class << self
            def query
              QueryRoot
            end
          end
        end

        e = assert_raises(StandardError) do
          klass.send(:definition)
        end

        assert_equal(
          "This is a test",
          e.message,
        )
      end
    end
  end
end
