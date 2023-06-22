# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class QueryRoot < ::Bluejay::QueryRoot
      class << self
        extend(T::Sig)
        include(DefinitionGuard)
        include(NameFromClass)

        sig { override.params(new_name: T.nilable(String)).returns(String) }
        def graphql_name(new_name = nil)
          guard_definition(new_name)
          @graphql_name ||= T.let(name_from_class, T.nilable(String))
          @graphql_name = new_name unless new_name.nil?
          @graphql_name
        end

        sig { override.params(new_description: T.nilable(String)).returns(T.nilable(String)) }
        def description(new_description = nil)
          guard_definition(new_description)
          @description ||= T.let(nil, T.nilable(String))
          @description = new_description unless new_description.nil?
          @description
        end

        sig { override.returns(T::Array[FieldDefinition]) }
        def field_definitions
          @field_definitions ||= T.let([], T.nilable(T::Array[FieldDefinition]))
        end

        sig do
          params(
            name: Symbol,
            type: T.untyped,
            null: T::Boolean,
            description: T.nilable(String),
          ).void
        end
        def field(name, type, null: true, description: nil)
          field_definition = FieldDefinition.new(
            resolver_method_name: name,
            type: output_type(type, null),
            description:,
          )
          guard_definition(field_definition)
          field_definitions.push(field_definition)
        end

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
              type < ::Bluejay::ObjectType ||
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
end
