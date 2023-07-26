# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module EnumType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(EnumTypeDefinition) }
      def definition; end
    end
  end
end
