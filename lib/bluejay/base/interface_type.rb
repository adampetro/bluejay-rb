# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module InterfaceType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(InterfaceTypeDefinition) }
      def definition; end
    end
  end
end
