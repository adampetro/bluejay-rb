# typed: ignore
# frozen_string_literal: true

require_relative "bluejay"

module Schemas
  module Models
    class DraftPosition < T::Struct
      extend(T::Sig)
      include(Schemas::Bluejay::DraftPosition::Interface)

      const(:round, Integer)
      const(:selection, Integer)
      const(:year, Integer)

      alias_method(:resolve_round, :round)
      alias_method(:resolve_selection, :selection)
      alias_method(:resolve_year, :year)
    end

    class Player < T::Struct
      extend(T::Sig)
      include(Schemas::Bluejay::Player::Interface)

      const(:first_name, String)
      const(:last_name, String)
      const(:age, Integer)
      const(:draft_position, T.nilable(DraftPosition))

      alias_method(:resolve_first_name, :first_name)
      alias_method(:resolve_last_name, :last_name)
      alias_method(:resolve_age, :age)
      alias_method(:resolve_draft_position, :draft_position)
    end

    class Team < T::Struct
      extend(T::Sig)
      include(Schemas::Bluejay::Team::Interface)

      const(:name, String)
      const(:city, String)
      const(:players, T::Array[Player])

      class << self
        extend(T::Sig)

        sig { returns(T::Array[Team]) }
        def all
          [
            Team.new(
              name: "Maple Leafs",
              city: "Toronto",
              players: [
                Player.new(
                  first_name: "Auston",
                  last_name: "Matthews",
                  age: 25,
                  draft_position: DraftPosition.new(
                    round: 1,
                    selection: 1,
                    year: 2016,
                  ),
                ),
                Player.new(
                  first_name: "Mitch",
                  last_name: "Marner",
                  age: 25,
                  draft_position: DraftPosition.new(
                    round: 1,
                    selection: 4,
                    year: 2015,
                  ),
                ),
                Player.new(
                  first_name: "William",
                  last_name: "Nylander",
                  age: 26,
                  draft_position: DraftPosition.new(
                    round: 1,
                    selection: 8,
                    year: 2014,
                  ),
                ),
                Player.new(
                  first_name: "John",
                  last_name: "Tavares",
                  age: 32,
                  draft_position: DraftPosition.new(
                    round: 1,
                    selection: 1,
                    year: 2009,
                  ),
                ),
              ],
            ),
            Team.new(
              name: "Bruins",
              city: "Boston",
              players: [
                Player.new(
                  first_name: "Patrice",
                  last_name: "Bergeron",
                  age: 37,
                ),
                Player.new(
                  first_name: "Brad",
                  last_name: "Marchand",
                  age: 34,
                ),
                Player.new(
                  first_name: "David",
                  last_name: "Pastrňák",
                  age: 26,
                ),
                Player.new(
                  first_name: "Charlie",
                  last_name: "McAvoy",
                  age: 25,
                ),
              ],
            ),
            Team.new(
              name: "Canadiens",
              city: "Montréal",
              players: [
                Player.new(
                  first_name: "Nick",
                  last_name: "Suzuki",
                  age: 23,
                ),
                Player.new(
                  first_name: "Cole",
                  last_name: "Caufield",
                  age: 22,
                ),
                Player.new(
                  first_name: "Kirby",
                  last_name: "Dach",
                  age: 21,
                ),
                Player.new(
                  first_name: "Sean",
                  last_name: "Monahan",
                  age: 28,
                ),
              ],
            ),
            Team.new(
              name: "Blackhawks",
              city: "Chicago",
              players: [],
            ),
            Team.new(
              name: "Red Wings",
              city: "Detroit",
              players: [],
            ),
            Team.new(
              name: "Rangers",
              city: "New York",
              players: [
                Player.new(
                  first_name: "Adam",
                  last_name: "Fox",
                  age: 24,
                ),
                Player.new(
                  first_name: "Artemi",
                  last_name: "Panarin",
                  age: 31,
                ),
                Player.new(
                  first_name: "Alexis",
                  last_name: "Lafrenière",
                  age: 21,
                ),
                Player.new(
                  first_name: "Igor",
                  last_name: "Shesterkin",
                  age: 27,
                ),
              ],
            ),
          ]
        end
      end

      alias_method(:resolve_name, :name)
      alias_method(:resolve_city, :city)
      alias_method(:resolve_players, :players)
    end

    class QueryRoot < T::Struct
      extend(T::Sig)
      include(Schemas::Bluejay::QueryRoot::Interface)

      const(:teams, T::Array[Team])

      alias_method(:resolve_teams, :teams)
    end

    class SchemaRoot < T::Struct
      extend(T::Sig)
      include(Schemas::Bluejay::Schema::Root)

      const(:query, QueryRoot)
    end
  end
end
