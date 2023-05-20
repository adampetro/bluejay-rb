# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module EnumTypes
      class TypeKind < EnumType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__TypeKind"
          end

          sig { override.returns(T::Array[EnumValueDefinition]) }
          def enum_value_definitions
            [
              EnumValueDefinition.new(name: "SCALAR"),
              EnumValueDefinition.new(name: "OBJECT"),
              EnumValueDefinition.new(name: "INTERFACE"),
              EnumValueDefinition.new(name: "UNION"),
              EnumValueDefinition.new(name: "ENUM"),
              EnumValueDefinition.new(name: "INPUT_OBJECT"),
              EnumValueDefinition.new(name: "LIST"),
              EnumValueDefinition.new(name: "NON_NULL"),
            ]
          end
        end
      end
    end
  end
end
