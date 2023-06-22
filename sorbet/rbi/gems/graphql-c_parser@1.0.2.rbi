# typed: true

# DO NOT EDIT MANUALLY
# This is an autogenerated file for types exported from the `graphql-c_parser` gem.
# Please instead update this file by running `bin/tapioca gem graphql-c_parser`.

# source://graphql-c_parser//lib/graphql/c_parser/version.rb#3
module GraphQL
  class << self
    # source://graphql/2.0.22/lib/graphql.rb#35
    def default_parser; end

    # source://graphql/2.0.22/lib/graphql.rb#39
    def default_parser=(_arg0); end

    # source://graphql/2.0.22/lib/graphql.rb#45
    def parse(graphql_string, trace: T.unsafe(nil)); end

    # source://graphql/2.0.22/lib/graphql.rb#52
    def parse_file(filename); end

    # source://graphql-c_parser//lib/graphql/c_parser.rb#77
    def parse_with_c(string, filename: T.unsafe(nil), trace: T.unsafe(nil)); end

    # source://graphql/2.0.22/lib/graphql.rb#62
    def parse_with_racc(string, filename: T.unsafe(nil), trace: T.unsafe(nil)); end

    # source://graphql/2.0.22/lib/graphql.rb#58
    def scan(graphql_string); end

    # source://graphql-c_parser//lib/graphql/c_parser.rb#73
    def scan_with_c(graphql_string); end

    # source://graphql/2.0.22/lib/graphql.rb#66
    def scan_with_ruby(graphql_string); end
  end
end

# source://graphql-c_parser//lib/graphql/c_parser/version.rb#4
module GraphQL::CParser
  class << self
    # source://graphql-c_parser//lib/graphql/c_parser.rb#9
    def parse(query_str, filename: T.unsafe(nil), trace: T.unsafe(nil)); end

    # source://graphql-c_parser//lib/graphql/c_parser.rb#14
    def parse_file(filename); end

    # source://graphql-c_parser//lib/graphql/c_parser.rb#19
    def prepare_parse_error(message, parser); end
  end
end

module GraphQL::CParser::Lexer
  class << self
    def tokenize(_arg0); end
  end
end

# source://graphql-c_parser//lib/graphql/c_parser.rb#43
class GraphQL::CParser::Parser
  # @return [Parser] a new instance of Parser
  #
  # source://graphql-c_parser//lib/graphql/c_parser.rb#44
  def initialize(query_string, filename, trace); end

  def c_parse; end

  # Returns the value of attribute filename.
  #
  # source://graphql-c_parser//lib/graphql/c_parser.rb#69
  def filename; end

  # Returns the value of attribute next_token_index.
  #
  # source://graphql-c_parser//lib/graphql/c_parser.rb#69
  def next_token_index; end

  # Returns the value of attribute query_string.
  #
  # source://graphql-c_parser//lib/graphql/c_parser.rb#69
  def query_string; end

  # source://graphql-c_parser//lib/graphql/c_parser.rb#56
  def result; end

  # Returns the value of attribute tokens.
  #
  # source://graphql-c_parser//lib/graphql/c_parser.rb#69
  def tokens; end
end

# source://graphql-c_parser//lib/graphql/c_parser/version.rb#5
GraphQL::CParser::VERSION = T.let(T.unsafe(nil), String)

# source://graphql/2.0.22/lib/graphql/version.rb#3
GraphQL::VERSION = T.let(T.unsafe(nil), String)