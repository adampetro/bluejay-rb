# typed: ignore
# frozen_string_literal: true

require_relative "bench"

module Legacy
  class DraftPosition < GraphQL::Schema::Object
    field(:round, Integer, null: false)
    field(:selection, Integer, null: false)
    field(:year, Integer, null: false)
  end

  class Player < GraphQL::Schema::Object
    field(:first_name, String, null: false)
    field(:last_name, String, null: false)
    field(:age, Integer, null: false)
    field(:draft_position, DraftPosition, null: true)
  end

  class Team < GraphQL::Schema::Object
    field(:name, String, null: false)
    field(:city, String, null: false)
    field(:players, [Player], null: false)
  end

  class QueryRoot < GraphQL::Schema::Object
    field(:teams, [Team], null: false)
  end

  class Schema < GraphQL::Schema
    query(QueryRoot)
  end
end

module New
  class DraftPosition < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "round", type: ot!(Bluejay::Scalar::Int)),
          Bluejay::FieldDefinition.new(name: "selection", type: ot!(Bluejay::Scalar::Int)),
          Bluejay::FieldDefinition.new(name: "year", type: ot!(Bluejay::Scalar::Int)),
        ]
      end
    end
  end

  class Player < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "firstName", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "lastName", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "age", type: ot!(Bluejay::Scalar::Int)),
          Bluejay::FieldDefinition.new(name: "draftPosition", type: ot(DraftPosition)),
        ]
      end
    end
  end

  class Team < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "name", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "city", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "players", type: lot!(ot!(Player))),
        ]
      end
    end
  end

  class QueryRoot < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "teams", type: lot!(ot!(Team))),
        ]
      end
    end
  end

  class Schema < Bluejay::Schema
    class << self
      extend(T::Sig)

      sig { override.returns(T.class_of(Bluejay::ObjectType)) }
      def query
        QueryRoot
      end
    end
  end
end

module Domain
  class DraftPosition < T::Struct
    extend(T::Sig)
    include(New::DraftPosition::Interface)

    const(:round, Integer)
    const(:selection, Integer)
    const(:year, Integer)

    sig { returns(Integer) }
    def graphql_round = round

    sig { returns(Integer) }
    def graphql_selection = selection

    sig { returns(Integer) }
    def graphql_year = year
  end

  class Player < T::Struct
    extend(T::Sig)
    include(New::Player::Interface)

    const(:first_name, String)
    const(:last_name, String)
    const(:age, Integer)
    const(:draft_position, T.nilable(DraftPosition))

    sig { returns(String) }
    def graphql_firstName = first_name

    sig { returns(String) }
    def graphql_lastName = last_name

    sig { returns(Integer) }
    def graphql_age = age

    sig { returns(T.nilable(DraftPosition)) }
    def graphql_draftPosition = draft_position
  end

  class Team < T::Struct
    extend(T::Sig)
    include(New::Team::Interface)

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

    sig { returns(String) }
    def graphql_name = name

    sig { returns(String) }
    def graphql_city = city

    sig { returns(T::Array[Player]) }
    def graphql_players = players
  end

  class QueryRoot < T::Struct
    extend(T::Sig)
    include(New::QueryRoot::Interface)

    const(:teams, T::Array[Team])

    sig { returns(T::Array[Team]) }
    def graphql_teams
      teams
    end
  end

  class SchemaRoot < T::Struct
    extend(T::Sig)

    const(:query, QueryRoot)
  end
end

Bench.all do |x|
  root_value = Domain::QueryRoot.new(teams: Domain::Team.all)
  schema_root_value = Domain::SchemaRoot.new(query: root_value)
  query = <<~GQL
    {
      teams {
        __typename
        name
        # name1: name
        # name2: name
        # name3: name
        # name4: name
        # name5: name
        city
        # city1: city
        # city2: city
        # city3: city
        # city4: city
        # city5: city
        players {
          __typename
          firstName
          lastName
          age
          draftPosition { __typename round selection year }
        }
      }
    }
  GQL

  x.report(:graphql) do
    result = Legacy::Schema.execute(query, root_value:, validate: false)
    # puts result.to_h
    # raise "error" unless result.to_h == { "data" => { "teams" => [{ "name" => "Maple Leafs", "city" => "Toronto" }] } }
    # raise "error" unless result.to_h["errors"].nil?
  end

  x.report(:bluejay) do
    result = New::Schema.execute(query:, operation_name: nil, initial_value: schema_root_value)
    # raise "error" unless result.value == { "teams" => [{ "name" => "Maple Leafs", "city" => "Toronto" }]}
  end

  x.compare!
  # puts Legacy::Schema.execute(query, root_value:, validate: false).to_h
  # puts New::Schema.execute(query:, operation_name: nil, initial_value: root_value).value
end
