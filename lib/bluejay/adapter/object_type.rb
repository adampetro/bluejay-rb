# typed: strict
# frozen_string_literal: true

module Bluejay
  module Adapter
    class ObjectType < Base::ObjectType
      extend(T::Sig)

      sig { params(object: Object).void }
      def initialize(object)
        @object = object
        super()
      end

      sig { returns(Object) }
      attr_reader(:object)

      class << self
        extend(T::Sig)
        include(DefinitionGuard)
        include(NameFromClass)

        sig { params(new_name: T.nilable(String)).returns(String) }
        def graphql_name(new_name = nil)
          guard_definition(new_name)
          @graphql_name ||= T.let(name_from_class, T.nilable(String))
          @graphql_name = new_name unless new_name.nil?
          @graphql_name
        end

        sig { params(new_description: T.nilable(String)).returns(T.nilable(String)) }
        def description(new_description = nil)
          guard_definition(new_description)
          @description ||= T.let(new_description, T.nilable(String))
        end

        sig { returns(T::Array[FieldDefinitionBuilder]) }
        def fields
          @fields ||= T.let([], T.nilable(T::Array[FieldDefinitionBuilder]))
        end

        sig do
          params(
            name: Symbol,
            type: T.untyped,
            null: T::Boolean,
            description: T.nilable(String),
            method: T.nilable(Symbol),
            blk: T.nilable(T.proc.params(builder: FieldDefinitionBuilder).void),
          ).void
        end
        def field(name, type, null: true, description: nil, method: nil, &blk)
          builder = FieldDefinitionBuilder.new(
            name:,
            type:,
            null:,
            description:,
            method:,
          )
          guard_definition(builder)
          blk&.call(builder)
          fields.push(builder)
        end

        private

        sig { returns(T::Array[FieldDefinition]) }
        def builtin_field_definitions
          [Builtin.typename_field_definition]
        end

        sig { override.returns(ObjectTypeDefinition) }
        def definition
          @definition ||= T.let(nil, T.nilable(ObjectTypeDefinition))
          @definition ||= ObjectTypeDefinition.new(
            name: graphql_name,
            description:,
            field_definitions: builtin_field_definitions + fields.map { _1.build(self) },
            interface_implementations: [],
            directives: [],
            ruby_class: self,
          )
        end
      end
    end
  end
end
