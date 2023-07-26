# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module UnionType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(UnionTypeDefinition) }
      def definition; end
    end
  end
end
