# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"

module Tapioca
  module Compilers
    class InterfaceType < Tapioca::Dsl::Compiler
      extend T::Sig

      ConstantType = type_member { { fixed: T.class_of(Bluejay::InterfaceType) } }

      class << self
        extend(T::Sig)

        sig { override.returns(T::Enumerable[Module]) }
        def gather_constants
          all_classes.select { |c| c < Bluejay::InterfaceType }
        end
      end

      sig { override.void }
      def decorate
        root.create_path(constant.const_get(:Interface)) do |klass|
          constant.interface_implementations.each do |interface_implementation|
            interface = interface_implementation.interface.const_get(:Interface)
            klass.create_include(interface.name)
          end
        end
      end
    end
  end
end
