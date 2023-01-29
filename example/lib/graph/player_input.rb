# typed: strict
# frozen_string_literal: true

module Graph
  class PlayerInput < Bluejay::InputType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::InputValueDefinition]) }
      def input_field_definitions
        [
          Bluejay::InputValueDefinition.new(name: "firstName", type: it!(Bluejay::Scalar::String)),
          Bluejay::InputValueDefinition.new(name: "lastName", type: it!(Bluejay::Scalar::String)),
        ]
      end
    end
  end
end
