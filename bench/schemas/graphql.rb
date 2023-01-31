# typed: strict
# frozen_string_literal: true

require "graphql"

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

    class Schema < ::GraphQL::Schema
      query(QueryRoot)
    end
  end
end
