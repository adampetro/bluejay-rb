# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    class << self
      extend(T::Sig)

      sig { returns(FieldDefinition) }
      def typename_field_definition
        @typename_field_definition ||= T.let(
          FieldDefinition.new(
            name: "__typename",
            type: OutputType.new(
              type: Scalar::String,
              required: true,
            ),
            resolver_method_name: "resolve_typename",
          ),
          T.nilable(FieldDefinition),
        )
      end
    end
  end
end
