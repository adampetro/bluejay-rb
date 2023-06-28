# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class FieldDefinitionBuilder
      extend(T::Sig)

      sig do
        params(
          name: Symbol,
          type: Object,
          null: T::Boolean,
          method: T.nilable(Symbol),
          description: T.nilable(String),
        ).void
      end
      def initialize(name:, type:, null:, method:, description:)
        @name = name
        @type = T.let(output_type(type, null), OutputType)
        @method = method
        @description = description
        @arguments = T.let([], T::Array[InputValueDefinitionBuilder])
      end

      sig { params(owner: Class).returns(FieldDefinition) }
      def build(owner)
        resolver_strategy = if owner.method_defined?(@name)
          ResolverStrategy::DefinitionInstance
        else
          ResolverStrategy::Object
        end
        FieldDefinition.new(
          name: @name,
          type: @type,
          description: @description,
          resolver_strategy:,
          resolver_method_name: @method,
          argument_definitions: @arguments.map(&:build),
        )
      end

      sig { params(new_description: String).void }
      def description(new_description)
        @description = new_description
      end

      sig do
        params(
          name: Symbol,
          type: Object,
          required: T::Boolean,
          description: T.nilable(String),
          blk: T.nilable(T.proc.params(builder: InputValueDefinitionBuilder).void),
        ).void
      end
      def argument(name, type, required: false, description: nil, &blk)
        builder = InputValueDefinitionBuilder.new(
          name:,
          type:,
          required:,
          description:,
        )
        blk&.call(builder)
        @arguments << builder
      end

      private

      sig { params(type: T.untyped, null: T::Boolean).returns(OutputType) }
      def output_type(type, null)
        if type.is_a?(Array)
          raise "Arrays wrapping types must have length 1" unless type.length == 1

          return OutputType.list(type: output_type(type[0], false), required: !null)
        end

        base = if type == String
          ::Bluejay::Scalar::String
        elsif type == Integer
          ::Bluejay::Scalar::Int
        elsif type == Float
          ::Bluejay::Scalar::Float
        elsif type < ::Bluejay::EnumType ||
            type < ::Bluejay::Base::ObjectType ||
            type < ::Bluejay::UnionType ||
            type < ::Bluejay::InterfaceType ||
            type < ::Bluejay::CustomScalarType
          type
        else
          raise "Unknown output type: #{type}"
        end

        OutputType.new(type: base, required: !null)
      end
    end
  end
end
