# typed: strict

module Bluejay
  class InterfaceImplementation
    sig { params(interface: T.class_of(InterfaceType)).void }
    def initialize(interface); end
  end
end
