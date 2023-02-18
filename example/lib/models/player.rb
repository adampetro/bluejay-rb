# typed: strict
# frozen_string_literal: true

class Player < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Player::Interface)

  add_index(:id, unique: true)
  add_index(:current_team)

  sig { override.returns(String) }
  def resolve_first_name = first_name

  sig { override.returns(String) }
  def resolve_last_name = last_name

  sig { override.returns(T.nilable(Graph::Team::Interface)) }
  def resolve_current_team
    Team.find_by(id: current_team)
  end

  sig { override.returns(Date) }
  def resolve_birthday
    birthday
  end
end
