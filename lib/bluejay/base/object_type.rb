# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module ObjectType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      module ClassMethods
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        requires_ancestor { Class }

        sig { abstract.returns(ObjectTypeDefinition) }
        def definition; end
      end

      mixes_in_class_methods(ClassMethods)
    end
  end
end
