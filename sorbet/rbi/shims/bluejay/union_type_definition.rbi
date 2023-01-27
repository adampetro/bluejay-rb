# typed: strict

module Bluejay
  class UnionTypeDefinition
    sig { params(name: String, member_types: T::Array[UnionMemberType], description: T.nilable(String)).void }
    def initialize(name:, member_types:, description:); end
  end
end
