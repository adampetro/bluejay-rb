# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class Directive < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__Directive"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "name", type: ot!(Scalar::String)),
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(name: "locations", type: lot!(ot!(EnumTypes::DirectiveLocation))),
              FieldDefinition.new(name: "args", type: lot!(ot!(InputValue))),
              FieldDefinition.new(
                name: "isRepeatable",
                type: ot!(Scalar::Boolean),
                resolver_method_name: :repeatable?,
              ),
            ]
          end
        end
      end
    end
  end
end
