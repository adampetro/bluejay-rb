# typed: ignore
# frozen_string_literal: true

require "bluejay"

module Schemas
  module Bluejay
    class DraftPosition < ::Bluejay::ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(name: "round", type: ot!(::Bluejay::Scalar::Int)),
            ::Bluejay::FieldDefinition.new(name: "selection", type: ot!(::Bluejay::Scalar::Int)),
            ::Bluejay::FieldDefinition.new(name: "year", type: ot!(::Bluejay::Scalar::Int)),
          ]
        end
      end
    end

    class Player < ::Bluejay::ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(name: "firstName", type: ot!(::Bluejay::Scalar::String)),
            ::Bluejay::FieldDefinition.new(name: "lastName", type: ot!(::Bluejay::Scalar::String)),
            ::Bluejay::FieldDefinition.new(name: "age", type: ot!(::Bluejay::Scalar::Int)),
            ::Bluejay::FieldDefinition.new(name: "draftPosition", type: ot(DraftPosition)),
          ]
        end
      end
    end

    class Team < ::Bluejay::ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(name: "name", type: ot!(::Bluejay::Scalar::String)),
            ::Bluejay::FieldDefinition.new(name: "city", type: ot!(::Bluejay::Scalar::String)),
            ::Bluejay::FieldDefinition.new(name: "players", type: lot!(ot!(Player))),
          ]
        end
      end
    end

    class QueryRoot < ::Bluejay::QueryRoot
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(name: "teams", type: lot!(ot!(Team))),
          ]
        end
      end
    end

    class Schema < ::Bluejay::Schema
      class << self
        extend(T::Sig)

        sig { override.returns(T.class_of(::Bluejay::QueryRoot)) }
        def query
          QueryRoot
        end
      end
    end
  end
end
