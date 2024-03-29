# typed: true

# DO NOT EDIT MANUALLY
# This is an autogenerated file for dynamic methods in `Graph::Player`.
# Please instead update this file by running `bin/tapioca dsl Graph::Player`.

module Graph::Player::Interface
  include Graph::Person::Interface

  abstract!

  sig { abstract.returns(Date) }
  def birthday; end

  sig { abstract.returns(String) }
  def first_name; end

  sig { abstract.returns(String) }
  def last_name; end

  sig { abstract.returns(T.nilable(Graph::Team::Interface)) }
  def resolve_current_team; end

  sig(:final) { returns(String) }
  def resolve_typename; end
end
