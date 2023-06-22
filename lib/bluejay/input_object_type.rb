# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputObjectType
    extend(T::Sig)

    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(InputTypeShorthands)
      include(NameFromClass)

      abstract!

      sig { overridable.returns(String) }
      def graphql_name
        name_from_class
      end

      sig { overridable.returns(T.nilable(String)) }
      def description
        nil
      end

      sig { abstract.returns(T::Array[InputValueDefinition]) }
      def input_field_definitions; end

      sig { overridable.returns(T::Array[Directive]) }
      def directives
        []
      end

      sig { params(value: T.untyped).returns(Result[T.untyped, T::Array[CoercionError]]) }
      def coerce_input(value)
        definition.coerce_input(value)
      end

      private

      sig(:final) { returns(InputObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(InputObjectTypeDefinition))
        @definition ||= begin
          input_field_definitions = self.input_field_definitions
          input_field_definitions.each { |ivd| attr_reader(ivd.ruby_name) }
          InputObjectTypeDefinition.new(
            name: graphql_name,
            input_field_definitions:,
            description:,
            directives:,
            ruby_class: self,
          )
        end
      end
    end

    define_method(:initialize) do |**kwargs|
      self.class.send(:definition).input_field_definitions.each do |ivd|
        arg = kwargs[ivd.ruby_name]
        instance_variable_set("@#{ivd.ruby_name}", arg)
      end
      freeze
    end

    define_method(:==) do |other|
      self.class == other.class && self.class.send(:definition).input_field_definitions.all? do |ivd|
        send(ivd.ruby_name) == other.send(ivd.ruby_name)
      end
    end
  end
end
