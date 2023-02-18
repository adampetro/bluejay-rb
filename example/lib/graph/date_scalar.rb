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

      sig { override.returns(String) }
      def internal_representation_sorbet_type_name
        "Date"
      end
    end
  end
end
