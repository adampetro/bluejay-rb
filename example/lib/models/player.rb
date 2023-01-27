# typed: strict
# frozen_string_literal: true

class Player < FrozenRecord::Base
  extend(T::Sig)
  include(Graph::Player::Interface)

  add_index(:id, unique: true)
  add_index(:current_team)

  sig { returns(String) }
  def graphql_firstName = first_name

  sig { returns(String) }
  def graphql_lastName = last_name
end
