# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class Type < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__Type"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "kind", type: ot!(EnumTypes::TypeKind)),
              FieldDefinition.new(name: "name", type: ot(Scalar::String)),
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(
                name: "fields",
                # TODO: default value
                argument_definitions: [InputValueDefinition.new(name: "includeDeprecated", type: it(Scalar::Boolean))],
                type: lot(ot!(Field)),
              ),
              FieldDefinition.new(name: "interfaces", type: lot(ot!(Type))),
              FieldDefinition.new(name: "possibleTypes", type: lot(ot!(Type))),
              FieldDefinition.new(
                name: "enumValues",
                # TODO: default value
                argument_definitions: [InputValueDefinition.new(name: "includeDeprecated", type: it(Scalar::Boolean))],
                type: lot(ot!(EnumValue)),
              ),
              FieldDefinition.new(
                name: "inputFields",
                type: lot(ot!(InputValue)),
              ),
              FieldDefinition.new(name: "ofType", type: ot(Type)),
              FieldDefinition.new(
                name: "specifiedByURL",
                type: ot(Scalar::String),
              ),
            ]
          end
        end
      end
    end
  end
end
