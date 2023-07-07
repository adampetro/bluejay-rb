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

    class DraftPositionInput < ::Bluejay::InputObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::InputValueDefinition]) }
        def input_field_definitions
          [
            ::Bluejay::InputValueDefinition.new(name: "round", type: it!(::Bluejay::Scalar::Int)),
            ::Bluejay::InputValueDefinition.new(name: "selection", type: it!(::Bluejay::Scalar::Int)),
            ::Bluejay::InputValueDefinition.new(name: "year", type: it!(::Bluejay::Scalar::Int)),
          ]
        end
      end
    end

    class PlayerInput < ::Bluejay::InputObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::InputValueDefinition]) }
        def input_field_definitions
          [
            ::Bluejay::InputValueDefinition.new(name: "firstName", type: it!(::Bluejay::Scalar::String)),
            ::Bluejay::InputValueDefinition.new(name: "lastName", type: it!(::Bluejay::Scalar::String)),
            ::Bluejay::InputValueDefinition.new(name: "age", type: it!(::Bluejay::Scalar::Int)),
            ::Bluejay::InputValueDefinition.new(name: "draftPosition", type: it(DraftPositionInput)),
          ]
        end
      end
    end

    class PlayersCreate < ::Bluejay::ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(name: "count", type: ot!(::Bluejay::Scalar::Int)),
          ]
        end
      end
    end

    class MutationRoot < ::Bluejay::ObjectType
      class << self
        extend(T::Sig)

        sig { override.returns(T::Array[::Bluejay::FieldDefinition]) }
        def field_definitions
          [
            ::Bluejay::FieldDefinition.new(
              name: "playersCreate",
              argument_definitions: [
                ::Bluejay::InputValueDefinition.new(
                  name: "players",
                  type: lit!(it!(PlayerInput)),
                ),
              ],
              type: ot!(PlayersCreate),
            ),
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

        sig { override.returns(T.nilable(T.class_of(::Bluejay::ObjectType))) }
        def mutation
          MutationRoot
        end
      end
    end
  end
end
