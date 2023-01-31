# frozen_string_literal: true

namespace :bench do
  Dir.glob("bench/*.rb").each do |path|
    task_name = File.basename(path, ".rb")
    next if task_name == "bench" # Bench helper

    desc "Run #{path} benchmark"
    task task_name do
      sh "ruby -Ilib #{path}"
      puts
    end
  end
end
