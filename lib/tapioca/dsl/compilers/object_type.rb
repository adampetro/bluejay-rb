# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"
require_relative "helper"

module Tapioca
  module Compilers
    class ObjectType < Tapioca::Dsl::Compiler
      extend T::Sig
      include(Helper)

      ConstantType = type_member {{ fixed: T.class_of(Bluejay::ObjectType) }}

      sig { override.returns(T::Enumerable[Module]) }
      def self.gather_constants
        all_classes.select { |c| c < Bluejay::ObjectType }
      end

      sig { override.void }
      def decorate
        root.create_path(constant.const_get(:Interface)) do |klass|
          klass.create_method("resolve_typename", return_type: "String", is_final: true)

          klass.mark_abstract

          constant.field_definitions.each do |field_definition|
            parameters = field_definition.argument_definitions.map do |argument_definition|
              create_param(argument_definition.name, type: argument_definition.type.sorbet_type)
            end

            return_type = field_definition.type.sorbet_type

            klass.create_method(field_definition.resolver_method_name, parameters:, return_type:, is_abstract: true)

            # klass.create_method(attr_name, return_type: "String")
            # klass.create_method("#{attr_name}=", parameters: [ create_param("value", type: "String") ], return_type: "void")
            # klass.create_method("#{attr_name}_encrypted", return_type: "String")
            # klass.create_method("#{attr_name}_encrypted=", parameters: [ create_param("value", type: "String") ], return_type: "void")

          end
        end
      end
    end
  end
end
