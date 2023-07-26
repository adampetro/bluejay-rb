# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module ObjectType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(ObjectTypeDefinition) }
      def definition; end
    end
  end
end
