# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module ObjectTypes
      class Schema < ObjectType
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "__Schema"
          end

          sig { override.returns(T::Array[FieldDefinition]) }
          def field_definitions
            [
              FieldDefinition.new(name: "description", type: ot(Scalar::String)),
              FieldDefinition.new(name: "types", type: lot!(ot!(Type))),
              FieldDefinition.new(name: "queryType", type: ot!(Type)),
              FieldDefinition.new(name: "mutationType", type: ot(Type)),
              FieldDefinition.new(name: "subscriptionType", type: ot(Type)),
              FieldDefinition.new(name: "directives", type: lot!(ot!(Directive))),
            ]
          end
        end
      end
    end
  end
end
