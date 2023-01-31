# typed: false
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestObjectType < Minitest::Test
    class MyObjectType < ObjectType
      extend(T::Sig)

      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[FieldDefinition]) }
        def field_definitions
          [
            FieldDefinition.new(name: "myField", type: ot!(Scalar::String)),
          ]
        end
      end
    end

    def test_foo
      refute_nil(MyObjectType.send(:definition))
    end
  end
end
