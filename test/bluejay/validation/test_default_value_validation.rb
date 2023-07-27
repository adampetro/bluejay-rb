# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  module Validation
    class TestDefaultValueValidation < Minitest::Test
      class QueryRoot < Bluejay::QueryRoot
        class << self
          extend(T::Sig)

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(
                name: "myField",
                argument_definitions: [
                  InputValueDefinition.new(
                    name: "myArgumentWithInvalidDefaultValue",
                    type: it!(Scalar::String),
                    default_value: 1,
                  ),
                  InputValueDefinition.new(
                    name: "myArgumentWithValidSymbolDefaultValue",
                    type: it!(Scalar::String),
                    default_value: :a_symbol,
                  ),
                  InputValueDefinition.new(
                    name: "myArgumentWithValidBigDecimalDefaultValue",
                    type: it!(Scalar::Float),
                    default_value: BigDecimal("1.0"),
                  ),
                ],
                type: ot!(Scalar::Int),
              ),
            ]
          end
        end
      end

      def test_default_value_validation
        klass = Class.new(Schema) do
          class << self
            def query
              QueryRoot
            end
          end
        end

        e = assert_raises(Errors::DefaultValueError) do
          klass.send(:definition)
        end

        assert_equal(
          <<~ERROR.chomp,
            Invalid default value `1` on input value definition `myArgumentWithInvalidDefaultValue`. Errors:
            No implicit conversion of integer to String
          ERROR
          e.message,
        )
      end
    end
  end
end
