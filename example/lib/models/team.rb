# typed: strict
# frozen_string_literal: true

class Team < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Team::Interface)

  add_index(:id, unique: true)

  sig { override.returns(T::Array[Player]) }
  def players
    Player.where(current_team: id).to_a
  end
end
