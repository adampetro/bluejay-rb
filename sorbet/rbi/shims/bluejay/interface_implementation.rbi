# typed: strict
# frozen_string_literal: true

module Bluejay
  class InterfaceImplementation
    sig { params(interface: Base::InterfaceType::ClassMethods, visibility: T.nilable(Visibility)).void }
    def initialize(interface:, visibility: nil); end

    sig { returns(Base::InterfaceType::ClassMethods) }
    def interface; end
  end
end
