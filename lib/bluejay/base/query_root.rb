# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module QueryRoot
      extend(T::Sig)
      extend(T::Helpers)
      include(OutputTypeShorthands)
      include(InputTypeShorthands)

      requires_ancestor { T.class_of(Base::ObjectType) }

      sig { returns(T::Array[FieldDefinition]) }
      def introspection_field_definitions
        [
          FieldDefinition.new(
            name: "__schema",
            type: ot!(Builtin::ObjectTypes::Schema),
            resolver_method_name: :resolve_schema,
            resolver_strategy: ResolverStrategy::DefinitionClass,
          ),
          FieldDefinition.new(
            name: "__type",
            argument_definitions: [InputValueDefinition.new(name: "name", type: it!(Scalar::String))],
            type: ot(Builtin::ObjectTypes::Type),
            resolver_method_name: :resolve_type,
            resolver_strategy: ResolverStrategy::DefinitionClass,
          ),
        ]
      end

      sig { params(schema_class: T.class_of(Schema)).returns(SchemaDefinition) }
      def resolve_schema(schema_class:)
        schema_class.send(:definition)
      end

      sig { params(name: String, schema_class: T.class_of(Schema)).returns(T.untyped) }
      def resolve_type(name:, schema_class:)
        schema_class.send(:definition).type(name)
      end
    end
  end
end
