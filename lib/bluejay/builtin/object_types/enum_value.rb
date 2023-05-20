# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class EnumValue < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__EnumValue"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "name", type: ot!(Scalar::String)),
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(
                name: "isDeprecated",
                type: ot!(Scalar::Boolean),
                resolver_method_name: "deprecated?",
              ),
              FieldDefinition.new(name: "deprecationReason", type: ot(Scalar::String)),
            ]
          end
        end
      end
    end
  end
end
