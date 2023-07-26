# typed: strict
# frozen_string_literal: true

module Bluejay
  module Visibility
    extend(T::Sig)
    extend(T::Helpers)

    interface!

    sig { abstract.returns(String) }
    def cache_key; end

    sig { abstract.params(context: T.untyped).returns(T::Boolean) }
    def visible?(context); end
  end
end
