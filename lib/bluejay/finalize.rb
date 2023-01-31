# typed: strict
# frozen_string_literal: true

module Bluejay
  module Finalize
    extend(T::Sig)
    extend(T::Helpers)

    abstract!

    sig { abstract.void }
    def finalize; end

    sig { params(obj: Finalize).void }
    def inherited(obj)
      TracePoint.trace(:end) do |t|
        if obj == t.self
          obj.finalize
          t.disable
        end
      end
      super
    end
  end

  private_constant(:Finalize)
end
