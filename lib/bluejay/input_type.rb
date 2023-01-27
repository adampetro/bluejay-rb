# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputType
    extend(Finalize)

    class << self
      extend(T::Sig)
      extend(T::Helpers)
      include(InputTypeReferenceShorthands)
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

      sig { params(value: T.untyped).returns(Result[T.untyped, T::Array[CoercionError]]) }
      def coerce_input(value)
        definition.coerce_input(value)
      end

      protected

      sig(:final) { override.void }
      def finalize
        definition
      end

      private

      sig(:final) { returns(InputObjectTypeDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(InputObjectTypeDefinition))
        @definition ||= begin
          self.define_method(:initialize) do |*args|
            self.class.send(:definition).input_field_definitions.zip(args) do |ivd, arg|
              self.instance_variable_set("@#{ivd.name}", arg)
            end
          end
          self.define_method(:==) do |other|
            self.class == other.class && self.class.send(:definition).input_field_definitions.all? do |ivd|
              self.send(ivd.name) == other.send(ivd.name)
            end
          end
          input_field_definitions = self.input_field_definitions
          input_field_definitions.each { |ivd| self.attr_reader(ivd.name) }
          InputObjectTypeDefinition.new(name: graphql_name, input_field_definitions:, description:, ruby_class: self)
        end
      end
    end
  end
end
