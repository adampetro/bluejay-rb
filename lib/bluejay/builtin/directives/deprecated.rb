# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module Directives
      class Deprecated < Directive
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "deprecated"
          end

          sig { override.returns(T::Array[InputValueDefinition]) }
          def argument_definitions
            [
              InputValueDefinition.new(name: "reason", type: it(Scalar::String)),
            ]
          end

          sig { override.returns(T::Array[DirectiveLocation]) }
          def locations
            [
              DirectiveLocation::FIELD_DEFINITION,
              DirectiveLocation::ENUM_VALUE,
            ]
          end
        end
      end
    end
  end
end
