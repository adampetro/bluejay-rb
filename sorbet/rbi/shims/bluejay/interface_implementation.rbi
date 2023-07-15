# typed: strict
# frozen_string_literal: true

module Bluejay
  class InterfaceImplementation
    sig { params(interface: Base::InterfaceType::ClassMethods).void }
    def initialize(interface); end

    sig { returns(Base::InterfaceType::ClassMethods) }
    def interface; end
  end
end
