# typed: strict
# frozen_string_literal: true

require "graphql"
require_relative "models"

module Schemas
  module GraphQL
    class DraftPosition < ::GraphQL::Schema::Object
      field(:round, Integer, null: false)
      field(:selection, Integer, null: false)
      field(:year, Integer, null: false)
    end

    class Player < ::GraphQL::Schema::Object
      field(:first_name, String, null: false)
      field(:last_name, String, null: false)
      field(:age, Integer, null: false)
      field(:draft_position, DraftPosition, null: true)
    end

    class Team < ::GraphQL::Schema::Object
      field(:name, String, null: false)
      field(:city, String, null: false)
      field(:players, [Player], null: false)
    end

    class QueryRoot < ::GraphQL::Schema::Object
      field(:teams, [Team], null: false)
    end

    class DraftPositionInput < ::GraphQL::Schema::InputObject
      argument(:round, Integer, required: true)
      argument(:selection, Integer, required: true)
      argument(:year, Integer, required: true)
    end

    class PlayerInput < ::GraphQL::Schema::InputObject
      argument(:first_name, String, required: true)
      argument(:last_name, String, required: true)
      argument(:age, Integer, required: true)
      argument(:draft_position, DraftPositionInput, required: false)
    end

    class PlayersCreate < ::GraphQL::Schema::Mutation
      extend(T::Sig)

      null(false)
      argument(:players, [PlayerInput], required: true)

      field(:count, Integer, null: false)

      class Result < T::Struct
        const(:count, Integer)
      end

      sig { params(players: T::Array[PlayerInput]).returns(T.untyped) }
      def resolve(players:)
        Result.new(count: players.length)
      end
    end

    class MutationRoot < ::GraphQL::Schema::Object
      field(:players_create, mutation: PlayersCreate)
    end

    class Schema < ::GraphQL::Schema
      query(QueryRoot)
      mutation(MutationRoot)
    end
  end
end
