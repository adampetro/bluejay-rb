# typed: strict
# frozen_string_literal: true

class SchemaRoot
  class << self
    extend(T::Sig)
  
    sig { returns(T.class_of(QueryRoot)) }
    def query
      QueryRoot
    end
  end
end
