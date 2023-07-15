# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module QueryRoot
      extend(T::Sig)
      extend(T::Helpers)

      requires_ancestor { ObjectType }
      interface!

      module ClassMethods
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        requires_ancestor { Class }
        requires_ancestor { ObjectType::ClassMethods }
      end

      mixes_in_class_methods(ClassMethods)
    end
  end
end
