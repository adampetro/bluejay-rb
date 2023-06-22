# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    module DefinitionGuard
      extend(T::Sig)
      extend(T::Helpers)

      requires_ancestor { Kernel }

      sig { type_parameters(:T).params(new_value: T.nilable(T.type_parameter(:T))).void }
      def guard_definition(new_value)
        case new_value
        when nil then nil
        else
          raise "Cannot mutate once definition is created" if defined?(@definition)
        end
      end
    end
  end
end
