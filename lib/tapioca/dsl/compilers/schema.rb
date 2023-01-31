# typed: strict
# frozen_string_literal: true

require_relative "../../../rbi_ext/model"

module Tapioca
  module Compilers
    class Schema < Tapioca::Dsl::Compiler
      extend T::Sig

      ConstantType = type_member { { fixed: T.class_of(Bluejay::Schema) } }

      class << self
        extend(T::Sig)

        sig { override.returns(T::Enumerable[Module]) }
        def gather_constants
          all_classes.select { |c| c < Bluejay::Schema }
        end
      end

      sig { override.void }
      def decorate
        root.create_path(constant) do |klass|
          parameters = [
            create_kw_param("query", type: "String"),
            create_kw_param("operation_name", type: "T.nilable(String)"),
            create_kw_param("initial_value", type: "#{constant.name}::Root"),
            create_kw_opt_param("variables", type: "T::Hash[String, T.untyped]", default: "{}"),
          ]
          klass.custom_create_method(
            "execute",
            return_type: "Bluejay::ExecutionResult",
            parameters:,
            class_method: true,
          )
        end

        root.create_path(constant.const_get(:Root)) do |klass|
          klass.mark_interface

          klass.custom_create_method("query", return_type: constant.query.const_get(:Interface).name, is_abstract: true)

          if (mutation = constant.mutation)
            klass.custom_create_method("mutation", return_type: mutation.const_get(:Interface).name, is_abstract: true)
          end
        end
      end
    end
  end
end
