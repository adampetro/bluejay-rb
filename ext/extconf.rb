# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("bluejay/bluejay_rb") do |ext|
  ext.extra_cargo_args += ["--package", "bluejay-rb"]
end
