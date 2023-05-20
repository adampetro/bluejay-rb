# typed: strict
# frozen_string_literal: true

class QueryRoot
  class << self
    extend(T::Sig)
    include(Graph::QueryRoot::Interface)

    sig { override.params(location: T.nilable(String)).returns(T::Array[Team]) }
    def teams(location)
      relation = if location
        Team.where(location:)
      else
        Team.all
      end
      relation.to_a
    end

    sig { override.returns(T::Array[Graph::Person::Interface]) }
    def people
      Player.all.to_a
    end
  end
end
