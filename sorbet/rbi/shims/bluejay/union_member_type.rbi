# typed: strict
# frozen_string_literal: true

module Bluejay
  class UnionMemberType
    sig { params(type: Base::ObjectType, visibility: T.nilable(Visibility)).void }
    def initialize(type:, visibility: nil); end
  end
end
