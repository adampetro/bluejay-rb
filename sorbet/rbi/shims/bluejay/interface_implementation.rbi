# typed: strict
# frozen_string_literal: true

module Bluejay
  class InterfaceImplementation
    sig { params(interface: Base::InterfaceType, visibility: T.nilable(Visibility)).void }
    def initialize(interface:, visibility: nil); end

    sig { returns(Base::InterfaceType) }
    def interface; end
  end
end
