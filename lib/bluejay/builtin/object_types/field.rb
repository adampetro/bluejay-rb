# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class Field < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__Field"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "name", type: ot!(Scalar::String)),
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(name: "args", type: lot!(ot!(InputValue))),
              FieldDefinition.new(name: "type", type: ot!(Type)),
              FieldDefinition.new(
                name: "isDeprecated",
                type: ot!(Scalar::Boolean),
                resolver_method_name: :deprecated?,
              ),
              FieldDefinition.new(name: "deprecationReason", type: ot(Scalar::String)),
            ]
          end
        end
      end
    end
  end
end
