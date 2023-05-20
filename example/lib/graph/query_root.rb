# typed: strict
# frozen_string_literal: true

module Graph
  class QueryRoot < Bluejay::QueryRoot
    class << self
      extend(T::Sig)

      sig { override.returns(T::Array[Bluejay::FieldDefinition]) }
      def field_definitions
        [
          Bluejay::FieldDefinition.new(
            name: "teams",
            type: lot!(ot!(Team)),
            argument_definitions: [
              Bluejay::InputValueDefinition.new(name: "location", type: it(Bluejay::Scalar::String)),
            ],
          ),
          Bluejay::FieldDefinition.new(name: "people", type: lot!(ot!(Person))),
        ]
      end
    end
  end
end
