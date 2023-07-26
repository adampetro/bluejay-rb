# frozen_string_literal: true

require_relative "bluejay/version"
require "sorbet-runtime"
require_relative "bluejay/finalize"
require_relative "bluejay/name_from_class"
require_relative "bluejay/visibility"
require_relative "bluejay/base"
require_relative "bluejay/base_input_type"
require_relative "bluejay/base_output_type"
require_relative "bluejay/input_type_shorthands"
require_relative "bluejay/output_type_shorthands"
require_relative "bluejay/json_value"
require_relative "bluejay/custom_scalar_type"
require_relative "bluejay/directive"
require_relative "bluejay/enum_type"
require_relative "bluejay/interface_type"
require_relative "bluejay/input_object_type"
require_relative "bluejay/object_type"
require_relative "bluejay/query_root"
require_relative "bluejay/schema"
require_relative "bluejay/union_type"
require_relative "bluejay/errors"
require_relative "bluejay/builtin"
require_relative "bluejay/builtin/directives/deprecated"
require_relative "bluejay/builtin/directives/include"
require_relative "bluejay/builtin/directives/skip"
require_relative "bluejay/builtin/directives/specified_by"
require_relative "bluejay/builtin/enum_types/directive_location"
require_relative "bluejay/builtin/enum_types/type_kind"
require_relative "bluejay/builtin/object_types/enum_value"
require_relative "bluejay/builtin/object_types/type"
require_relative "bluejay/builtin/object_types/input_value"
require_relative "bluejay/builtin/object_types/field"
require_relative "bluejay/builtin/object_types/directive"
require_relative "bluejay/builtin/object_types/schema"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "bluejay/#{Regexp.last_match(1)}/bluejay_rb"
rescue LoadError
  require "bluejay/bluejay_rb"
end

module Bluejay
end
