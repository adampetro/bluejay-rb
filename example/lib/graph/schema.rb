# typed: strict
# frozen_string_literal: true

module Graph
  class Schema < Bluejay::Schema
    class << self
      extend(T::Sig)

      sig { override.returns(T.class_of(Bluejay::ObjectType)) }
      def query = QueryRoot
    end
  end
end
