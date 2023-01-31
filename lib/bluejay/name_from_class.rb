# typed: strict
# frozen_string_literal: true

module Bluejay
  module NameFromClass
    extend(T::Sig)
    extend(T::Helpers)

    requires_ancestor { Class }

    private

    sig { returns(String) }
    def name_from_class
      name&.split("::")&.last || "Anonymous"
    end
  end
end
