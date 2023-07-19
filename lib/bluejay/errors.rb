# typed: strict
# frozen_string_literal: true

module Bluejay
  module Errors
    class BaseError < StandardError; end

    class DefaultValueError < BaseError
      extend(T::Sig)

      sig { params(errors: T::Array[CoercionError], value: Object).void }
      def initialize(errors, value)
        @errors = errors
        super("Invalid default value: #{value}. Errors:\n#{errors.map(&:message).join("\n")}")
      end
    end

    class NonUniqueDefinitionNameError < BaseError; end
  end
end
