# typed: strict
# frozen_string_literal: true

class Player < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Player::Interface)

  add_index(:id, unique: true)
  add_index(:current_team)

  sig { override.returns(T.nilable(Graph::Team::Interface)) }
  def resolve_current_team
    Team.find_by(id: current_team)
  end
end
