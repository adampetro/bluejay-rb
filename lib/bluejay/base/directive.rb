# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module Directive
      extend(T::Sig)
      extend(T::Helpers)

      module Instance
        extend(T::Sig)
        extend(T::Helpers)

        interface!

        mixes_in_class_methods(Directive)
      end

      interface!

      sig { abstract.returns(DirectiveDefinition) }
      def definition; end
    end
  end
end
