# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"

module Tapioca
  module Compilers
    class InputObjectType < Tapioca::Dsl::Compiler
      extend T::Sig

      ConstantType = type_member { { fixed: T.class_of(Bluejay::InputObjectType) } }

      class << self
        extend(T::Sig)

        sig { override.returns(T::Enumerable[Module]) }
        def gather_constants
          all_classes.select { |c| c < Bluejay::InputObjectType }
        end
      end

      sig { override.void }
      def decorate
        root.create_path(constant) do |klass|
          parameters = constant.input_field_definitions.map do |input_field_definition|
            create_kw_param(input_field_definition.ruby_name.to_s, type: input_field_definition.type.sorbet_type)
          end

          klass.custom_create_method("initialize", parameters:, return_type: nil)

          constant.input_field_definitions.each do |input_field_definition|
            klass.custom_create_method(
              input_field_definition.ruby_name.to_s,
              return_type: input_field_definition.type.sorbet_type,
            )
          end
        end
      end
    end
  end
end
