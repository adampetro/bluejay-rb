# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module CustomScalarType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      module ClassMethods
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        sig { abstract.returns(CustomScalarTypeDefinition) }
        def definition; end
      end

      mixes_in_class_methods(ClassMethods)
    end
  end
end
