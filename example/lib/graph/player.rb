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
          Bluejay::FieldDefinition.new(
            name: "currentTeam",
            type: ot(Team),
            resolver_method_name: "resolve_current_team",
          ),
          Bluejay::FieldDefinition.new(name: "birthday", type: ot!(DateScalar)),
        ]
      end

      sig { override.returns(T::Array[Bluejay::InterfaceImplementation]) }
      def interface_implementations
        [
          Bluejay::InterfaceImplementation.new(Person),
        ]
      end
    end
  end
end
