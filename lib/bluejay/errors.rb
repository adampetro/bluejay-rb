# typed: strict
# frozen_string_literal: true

module Bluejay
  module Errors
    class DefaultValueError < StandardError
      extend(T::Sig)

      sig { params(errors: T::Array[CoercionError], value: Object).void }
      def initialize(errors, value)
        @errors = errors
        super("Invalid default value: #{value}. Errors:\n #{errors.map(&:message).join("\n")}")
      end
    end
  end
end
