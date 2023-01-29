# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"

module Tapioca
  module Compilers
    class InputType < Tapioca::Dsl::Compiler
      extend T::Sig

      ConstantType = type_member { { fixed: T.class_of(Bluejay::InputType) } }

      sig { override.returns(T::Enumerable[Module]) }
      def self.gather_constants
        all_classes.select { |c| c < Bluejay::InputType }
      end

      sig { override.void }
      def decorate
        root.create_path(constant) do |klass|
          parameters = constant.input_field_definitions.map do |input_field_definition|
            create_param(input_field_definition.ruby_name, type: input_field_definition.type.sorbet_type)
          end

          klass.custom_create_method("initialize", parameters:, return_type: nil)

          constant.input_field_definitions.each do |input_field_definition|
            klass.custom_create_method(input_field_definition.ruby_name, return_type: input_field_definition.type.sorbet_type)
          end
        end
      end
    end
  end
end
