# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module Directives
      class SpecifiedBy < Directive
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "specifiedBy"
          end

          sig { override.returns(T::Array[InputValueDefinition]) }
          def argument_definitions
            [
              InputValueDefinition.new(name: "url", type: it!(Scalar::String)),
            ]
          end

          sig { override.returns(T::Array[DirectiveLocation]) }
          def locations
            [
              DirectiveLocation::SCALAR,
            ]
          end
        end
      end
    end
  end
end
