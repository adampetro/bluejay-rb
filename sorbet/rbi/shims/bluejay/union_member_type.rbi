# typed: strict

module Bluejay
  class UnionMemberType
    sig { params(type: T.class_of(ObjectType)).void }
    def initialize(type); end
  end
end
