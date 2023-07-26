# typed: strict
# frozen_string_literal: true

module Bluejay
  class UnionMemberType
    sig { params(type: Base::ObjectType).void }
    def initialize(type:); end
  end
end
