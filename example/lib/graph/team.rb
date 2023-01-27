# typed: strict
# frozen_string_literal: true

module Graph
  class Team < Bluejay::ObjectType
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(name: "name", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "location", type: ot!(Bluejay::Scalar::String)),
          Bluejay::FieldDefinition.new(name: "players", type: lot!(ot!(Player))),
        ]
      end
    end
  end
end
