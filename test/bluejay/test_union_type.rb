# typed: false
# frozen_string_literal: true

require "test_helper"

module Bluejay
  class TestUnionType < Minitest::Test
    class MyObjectType < ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(String) }
        def graphql_name
          "MyObjectType"
        end

        sig { override.returns(T::Array[FieldDefinition]) }
        def field_definitions
          [
            FieldDefinition.new(name: "myField", type: ot!(Scalar::String)),
          ]
        end
      end
    end

    class MyUnionType < UnionType
      class << self
        extend(T::Sig)

        sig { override.returns(String) }
        def graphql_name
          "MyUnionType"
        end

        sig { override.returns(T::Array[UnionMemberType]) }
        def member_types
          [
            UnionMemberType.new(MyObjectType),
          ]
        end
      end
    end

    def test_foo
      refute_nil(MyUnionType.send(:definition))
    end
  end
end
