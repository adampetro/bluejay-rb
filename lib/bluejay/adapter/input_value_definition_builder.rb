# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class InputValueDefinitionBuilder
      extend(T::Sig)

      sig do
        params(
          name: Symbol,
          type: Object,
          required: T::Boolean,
          description: T.nilable(String),
        ).void
      end
      def initialize(name:, type:, required:, description:)
        @name = name
        @type = T.let(input_type(type, required), InputType)
        @description = description
      end

      sig { returns(InputValueDefinition) }
      def build
        InputValueDefinition.new(
          name: @name,
          type: @type,
          description: @description,
        )
      end

      sig { params(new_description: String).void }
      def description(new_description)
        @description = new_description
      end

      private

      sig { params(type: T.untyped, required: T::Boolean).returns(InputType) }
      def input_type(type, required)
        if type.is_a?(Array)
          raise "Arrays wrapping types must have length 1" unless type.length == 1

          return InputType.list(type: input_type(type[0], false), required:)
        end

        base = if type == String
          ::Bluejay::Scalar::String
        elsif type == Integer
          ::Bluejay::Scalar::Int
        elsif type == Float
          ::Bluejay::Scalar::Float
        elsif type < ::Bluejay::EnumType ||
            type < ::Bluejay::InputObjectType ||
            type < ::Bluejay::CustomScalarType
          type
        else
          raise "Unknown input type: #{type}"
        end

        InputType.new(type: base, required:)
      end
    end
  end
end
