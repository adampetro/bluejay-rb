# frozen_string_literal: true

require_relative "bluejay/version"
require "sorbet-runtime"
require_relative "bluejay/finalize"
require_relative "bluejay/name_from_class"
require_relative "bluejay/base_input_type_reference"
require_relative "bluejay/base_output_type_reference"
require_relative "bluejay/input_type_reference_shorthands"
require_relative "bluejay/output_type_reference_shorthands"
require_relative "bluejay/json_value"
require_relative "bluejay/custom_scalar_type"
require_relative "bluejay/directive"
require_relative "bluejay/enum_type"
require_relative "bluejay/interface_type"
require_relative "bluejay/input_type"
require_relative "bluejay/object_type"
require_relative "bluejay/schema"
require_relative "bluejay/union_type"
require_relative "bluejay/builtin/directives/include"
require_relative "bluejay/builtin/directives/skip"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "bluejay/#{Regexp.last_match(1)}/ext"
rescue LoadError
  begin
    require "bluejay/ext"
  rescue LoadError
    require_relative "../ext/ext"
  end
end

module Bluejay
end
