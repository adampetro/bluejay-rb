# typed: strict
# frozen_string_literal: true

module Bluejay
  class Directive
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
      def argument_definitions; end

      sig { abstract.returns(T::Array[DirectiveLocation]) }
      def locations; end

      sig { overridable.returns(T::Boolean) }
      def repeatable?
        false
      end

      private

      sig(:final) { returns(DirectiveDefinition) }
      def definition
        @definition ||= T.let(nil, T.nilable(DirectiveDefinition))
        @definition ||= begin
          argument_definitions = self.argument_definitions
          argument_definitions.each { |ivd| attr_reader(ivd.ruby_name) }
          DirectiveDefinition.new(
            name: graphql_name,
            argument_definitions:,
            description:,
            locations:,
            is_repeatable: repeatable?,
            ruby_class: self,
          )
        end
      end
    end

    define_method(:initialize) do |*args|
      self.class.send(:definition).argument_definitions.zip(args) do |ivd, arg|
        instance_variable_set("@#{ivd.ruby_name}", arg)
      end
      freeze
    end

    define_method(:==) do |other|
      self.class == other.class && self.class.send(:definition).argument_definitions.all? do |ivd|
        send(ivd.ruby_name) == other.send(ivd.ruby_name)
      end
    end
  end
end
