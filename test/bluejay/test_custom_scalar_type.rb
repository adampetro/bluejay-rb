# typed: true
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestCustomScalarType < Minitest::Test
    class DateScalar < CustomScalarType
      extend(T::Generic)

      InternalRepresentation = type_template { { fixed: Date } }

      class << self
        extend(T::Sig)

        sig { override.returns(String) }
        def graphql_name = "Date"

        sig { override.params(value: InternalRepresentation).returns(Result[T.untyped, String]) }
        def coerce_result(value)
          Result.ok(value.iso8601)
        end

        sig { override.params(value: T.untyped).returns(Result[Date, String]) }
        def coerce_input(value)
          raise NotImplementedError
        end
      end
    end

    def test_definition_exists
      refute_nil(DateScalar.send(:definition))
    end

    def test_coerce_result
      date = Date.new(2023, 1, 1)

      assert_equal(Result.ok("2023-01-01"), DateScalar.coerce_result(date))
    end
  end
end
