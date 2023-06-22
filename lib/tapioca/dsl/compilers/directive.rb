# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"
require "bluejay"

module Tapioca
  module Compilers
    class Directive < Tapioca::Dsl::Compiler
      extend T::Sig

      ConstantType = type_member { { fixed: T.class_of(Bluejay::Directive) } }

      class << self
        extend(T::Sig)

        sig { override.returns(T::Enumerable[Module]) }
        def gather_constants
          all_classes.select { |c| c < Bluejay::Directive }
        end
      end

      sig { override.void }
      def decorate
        root.create_path(constant) do |klass|
          parameters = constant.argument_definitions.map do |argument_definition|
            create_kw_param(argument_definition.ruby_name, type: argument_definition.type.sorbet_type)
          end

          klass.custom_create_method("initialize", parameters:, return_type: nil)

          constant.argument_definitions.each do |argument_definition|
            klass.custom_create_method(
              argument_definition.ruby_name,
              return_type: argument_definition.type.sorbet_type,
            )
          end
        end
      end
    end
  end
end
