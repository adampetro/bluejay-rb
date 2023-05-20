# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class InputValue < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__InputValue"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(name: "name", type: ot(Scalar::String)),
              FieldDefinition.new(name: "type", type: ot!(Type)),
              FieldDefinition.new(name: "defaultValue", type: ot(Scalar::String)),
            ]
          end
        end
      end
    end
  end
end
