# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    class ObjectType
      class << self
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        private

        sig { abstract.returns(ObjectTypeDefinition) }
        def definition; end
      end
    end
  end
end
