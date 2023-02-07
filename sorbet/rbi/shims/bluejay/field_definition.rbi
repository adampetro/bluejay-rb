# typed: strict

module Bluejay
  class FieldDefinition
    sig { params(name: String, type: OutputTypeReference, argument_definitions: T::Array[InputValueDefinition], description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, type:, argument_definitions: [], description: nil, directives: []); end

    sig { returns(String) }
    def name; end

    sig { returns(String) }
    def resolver_method_name; end

    sig { returns(T::Array[InputValueDefinition]) }
    def argument_definitions; end

    sig { returns(OutputTypeReference) }
    def type; end

    sig { returns(T::Array[Directive]) }
    def directives; end
  end
end
