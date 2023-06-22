# typed: true

# DO NOT EDIT MANUALLY
# This is an autogenerated file for dynamic methods in `Graph::QueryRoot`.
# Please instead update this file by running `bin/tapioca dsl Graph::QueryRoot`.

module Graph::QueryRoot::Interface
  abstract!

  sig { abstract.returns(T::Array[Graph::Person::Interface]) }
  def people; end

  sig(:final) { returns(String) }
  def resolve_typename; end

  sig { abstract.params(location: T.nilable(String)).returns(T::Array[Graph::Team::Interface]) }
  def teams(location:); end
end
