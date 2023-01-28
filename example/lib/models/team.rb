# typed: strict
# frozen_string_literal: true

class Team < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Team::Interface)

  add_index(:id, unique: true)

  sig { override.returns(String) }
  def resolve_location = location

  sig { override.returns(String) }
  def resolve_name = name

  sig { override.returns(T::Array[Player]) }
  def resolve_players
    Player.where(current_team: id).to_a
  end
end
