# typed: strict

module Bluejay
  class EnumValueDefinition
    sig { params(name: String, description: T.nilable(String), directives: T::Array[Directive]).void }
    def initialize(name:, description: nil, directives: []); end

    sig { returns(String) }
    def name; end
  end
end
