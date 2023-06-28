# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class QueryRoot < ObjectType
      class << self
        extend(T::Sig)
        include(Base::QueryRoot)

        private

        sig { returns(T::Array[FieldDefinition]) }
        def builtin_field_definitions
          super + introspection_field_definitions
        end
      end
    end
  end
end
