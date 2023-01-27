# typed: strict
# frozen_string_literal: true

class Team < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Team::Interface)

  add_index(:id, unique: true)

  sig { returns(String) }
  def graphql_location = location

  sig { returns(String) }
  def graphql_name = name

  sig { returns(T::Array[Player]) }
  def graphql_players
    Player.where(current_team: id).to_a
  end
end
