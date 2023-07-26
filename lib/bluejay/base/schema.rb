# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module Schema
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      sig { abstract.returns(SchemaDefinition) }
      def definition; end
    end
  end
end
