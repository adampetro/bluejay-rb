# typed: strict

module Bluejay
  class UnionTypeDefinition
    sig { params(name: String, member_types: T::Array[UnionMemberType], description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, member_types:, description:, directives:); end
  end
end
