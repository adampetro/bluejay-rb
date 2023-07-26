# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module InputObjectType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(InputObjectTypeDefinition) }
      def definition; end
    end
  end
end
