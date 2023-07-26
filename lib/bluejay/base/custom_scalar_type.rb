# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module CustomScalarType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(CustomScalarTypeDefinition) }
      def definition; end
    end
  end
end
