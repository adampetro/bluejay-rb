# typed: strict

module Bluejay
  class EnumValueDefinition
    sig { params(name: String, description: T.nilable(String), directives: T::Array[Directive], deprecation_reason: T.nilable(String)).void }
    def initialize(name:, description: nil, directives: [], deprecation_reason: nil); end

    sig { returns(String) }
    def name; end
  end
end
