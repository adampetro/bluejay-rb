# typed: ignore
# frozen_string_literal: true

require_relative "bench"
require_relative "schemas/bluejay"
require_relative "schemas/graphql"
require_relative "schemas/models"

Bench.all do |x|
  root_value = Schemas::Models::QueryRoot.new(teams: Schemas::Models::Team.all)
  schema_root_value = Schemas::Models::SchemaRoot.new(query: root_value)
  query = <<~GQL
    mutation PlayersCreate($players: [PlayerInput!]!) {
      playersCreate(players: $players) {
        count
      }
    }
  GQL
  variables = {
    "players" => root_value.teams.flat_map do |team|
      team.players.map do |player|
        {
          "firstName" => player.first_name,
          "lastName" => player.last_name,
          "age" => player.age,
          "draftPosition" => player.draft_position&.then do |draft_position|
            {
              "round" => draft_position.round,
              "selection" => draft_position.selection,
              "year" => draft_position.year,
            }
          end,
        }
      end
    end,
  }

  graphql_test_run = Schemas::GraphQL::Schema.execute(query, root_value:, variables:, validate: false)
  bluejay_test_run = Schemas::Bluejay::Schema.execute(query:, variables:, initial_value: schema_root_value)

  unless graphql_test_run.to_h["errors"].nil? && bluejay_test_run.errors.empty?
    raise "errors returned"
  end

  unless graphql_test_run.to_h["data"] == bluejay_test_run.value
    raise "results not equal"
  end

  x.report(:graphql) do
    Schemas::GraphQL::Schema.execute(query, root_value:, variables:, validate: false)
  end

  x.report(:bluejay) do
    Schemas::Bluejay::Schema.execute(query:, variables:, initial_value: schema_root_value)
  end

  x.compare!
end
