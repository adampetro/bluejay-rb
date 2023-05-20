# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module EnumTypes
      class DirectiveLocation < EnumType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__DirectiveLocation"
          end

          sig { override.returns(T::Array[EnumValueDefinition]) }
          def enum_value_definitions
            [
              EnumValueDefinition.new(name: "QUERY"),
              EnumValueDefinition.new(name: "MUTATION"),
              EnumValueDefinition.new(name: "SUBSCRIPTION"),
              EnumValueDefinition.new(name: "FIELD"),
              EnumValueDefinition.new(name: "FRAGMENT_DEFINITION"),
              EnumValueDefinition.new(name: "FRAGMENT_SPREAD"),
              EnumValueDefinition.new(name: "INLINE_FRAGMENT"),
              EnumValueDefinition.new(name: "VARIABLE_DEFINITION"),
              EnumValueDefinition.new(name: "SCHEMA"),
              EnumValueDefinition.new(name: "SCALAR"),
              EnumValueDefinition.new(name: "OBJECT"),
              EnumValueDefinition.new(name: "FIELD_DEFINITION"),
              EnumValueDefinition.new(name: "ARGUMENT_DEFINITION"),
              EnumValueDefinition.new(name: "INTERFACE"),
              EnumValueDefinition.new(name: "UNION"),
              EnumValueDefinition.new(name: "ENUM"),
              EnumValueDefinition.new(name: "ENUM_VALUE"),
              EnumValueDefinition.new(name: "INPUT_OBJECT"),
              EnumValueDefinition.new(name: "INPUT_FIELD_DEFINITION"),
            ]
          end
        end
      end
    end
  end
end
