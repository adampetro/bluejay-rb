# typed: ignore
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestFieldDefinition < Minitest::Test
    include(OutputTypeShorthands)

    class MyDirective < Directive
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[InputValueDefinition]) }
        def argument_definitions
          [
            InputValueDefinition.new(name: "myArg", type: it!(Scalar::String), description: "This is my arg"),
          ]
        end

        sig { override.returns(T::Array[DirectiveLocation]) }
        def locations
          [DirectiveLocation::FIELD_DEFINITION]
        end
      end
    end

    def test_resolver_method_name
      instance = FieldDefinition.new(name: "myField", type: ot!(Scalar::String))

      assert_equal("resolve_my_field", instance.resolver_method_name)
    end

    def test_directives
      directives = [MyDirective.new("test")]
      instance = FieldDefinition.new(name: "myField", type: ot!(Scalar::String), directives:)

      assert_equal(directives, instance.directives)
    end
  end
end
