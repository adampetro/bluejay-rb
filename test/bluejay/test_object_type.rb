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

    def test_definition_exists
      refute_nil(MyObjectType.send(:definition))
    end

    def test_interface_module_exists
      assert_instance_of(Module, MyObjectType.const_get(:Interface))
    end

    def test_const_missing
      assert_raises(NameError) { MyObjectType.const_get(:Foo) }
    end
  end
end
