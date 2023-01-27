# typed: strict

module Bluejay
  class FieldDefinition
    sig { params(name: String, type: OutputTypeReference, argument_definitions: T::Array[InputValueDefinition], description: T.nilable(String)).void }
    def initialize(name:, type:, argument_definitions: [], description: nil); end
  end
end
