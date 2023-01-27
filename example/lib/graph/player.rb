# typed: strict
# frozen_string_literal: true

module Graph
  class Player < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "firstName", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "lastName", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "currentTeam", type: ot(Team)),
        ]
      end
    end
  end
end
