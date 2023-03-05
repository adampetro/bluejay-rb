# typed: strict
# frozen_string_literal: true

module Graph
  class DateScalar < Bluejay::CustomScalarType
    extend(T::Generic)

    InternalRepresentation = type_template { { fixed: Date } }

    class << self
      extend(T::Sig)

      sig { override.returns(String) }
      def graphql_name = "Date"

      sig { override.params(value: InternalRepresentation).returns(Bluejay::Result[T.untyped, String]) }
      def coerce_result(value)
        Bluejay::Result.ok(value.iso8601)
      end

      sig { override.params(value: T.untyped).returns(Bluejay::Result[Date, String]) }
      def coerce_input(value)
        if value.is_a?(String)
          begin
            Bluejay::Result.ok(Date.parse(value))
          rescue Date::Error => e
            Bluejay::Result.err(e.message)
          end
        else
          Bluejay::Result.err("Expected a date encoded as a string")
        end
      end

      sig { override.returns(String) }
      def internal_representation_sorbet_type_name
        "Date"
      end
    end
  end
end
