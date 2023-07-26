# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module QueryRoot
      extend(T::Sig)
      extend(T::Helpers)

      requires_ancestor { ObjectType }
      interface!
    end
  end
end
