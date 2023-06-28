# typed: strict
# frozen_string_literal: true

module Bluejay
  module Builtin
    module Directives
      class Skip < Directive
        class << self
          extend(T::Sig)

          sig { override.returns(String) }
          def graphql_name
            "skip"
          end

          sig { override.returns(T::Array[InputValueDefinition]) }
          def argument_definitions
            [
              InputValueDefinition.new(name: "if", type: it!(Scalar::Boolean), ruby_name: :if_arg),
            ]
          end

          sig { override.returns(T::Array[DirectiveLocation]) }
          def locations
            [
              DirectiveLocation::FIELD,
              DirectiveLocation::FRAGMENT_SPREAD,
              DirectiveLocation::INLINE_FRAGMENT,
            ]
          end
        end
      end
    end
  end
end
