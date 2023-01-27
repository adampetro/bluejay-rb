# typed: strict

module Bluejay
  class << self
    sig { params(query: String).returns(T::Boolean) }
    def parse(query); end
  end
end
  