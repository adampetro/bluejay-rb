# typed: strict

module Bluejay
  class InterfaceImplementation
    sig { params(interface: T.class_of(InterfaceType)).void }
    def initialize(interface); end

    sig { returns(T.class_of(InterfaceType)) }
    def interface; end
  end
end
