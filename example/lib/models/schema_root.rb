# typed: strict
# frozen_string_literal: true

class SchemaRoot
  class << self
    extend(T::Sig)
    include(Graph::Schema::Root)
  
    sig { override.returns(T.class_of(QueryRoot)) }
    def query
      QueryRoot
    end
  end
end
