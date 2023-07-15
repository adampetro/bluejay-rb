# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module UnionType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      module ClassMethods
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        sig { abstract.returns(UnionTypeDefinition) }
        def definition; end
      end

      mixes_in_class_methods(ClassMethods)
    end
  end
end
