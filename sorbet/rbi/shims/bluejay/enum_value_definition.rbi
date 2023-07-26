# typed: strict
# frozen_string_literal: true

module Bluejay
  class EnumValueDefinition
    sig do
      params(
        name: String,
        description: T.nilable(String),
        directives: T::Array[Base::Directive::Instance],
        deprecation_reason: T.nilable(String),
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, description: nil, directives: [], deprecation_reason: nil, visibility: nil); end

    sig { returns(String) }
    def name; end
  end
end
