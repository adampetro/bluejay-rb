# typed: strict
# frozen_string_literal: true

module Bluejay
  module Errors
    class BaseError < StandardError; end

    class DefaultValueError < BaseError
      extend(T::Sig)

      sig { params(errors: T::Array[CoercionError], value: Object, ivd_name: String).void }
      def initialize(errors, value, ivd_name)
        @errors = errors
        super("Invalid default value `#{value}` on input value definition `#{ivd_name}`. Errors:\n"\
          "#{errors.map(&:message).join("\n")}")
      end
    end

    class NonUniqueDefinitionNameError < BaseError; end
  end
end
