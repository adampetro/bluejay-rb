# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    class Schema
      class << self
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        private

        sig { abstract.returns(SchemaDefinition) }
        def definition; end
      end
    end
  end
end
