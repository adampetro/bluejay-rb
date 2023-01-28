# typed: strict

module Bluejay
  class FieldDefinition
    sig { params(name: String, type: OutputTypeReference, argument_definitions: T::Array[InputValueDefinition], description: T.nilable(String)).void }
    def initialize(name:, type:, argument_definitions: [], description: nil); end

    sig { returns(String) }
    def name; end

    sig { returns(String) }
    def resolver_method_name; end
  end
end
