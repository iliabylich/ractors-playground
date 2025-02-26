# Single producer, multiple consumers

require_relative './helper.rb'

QUEUE = CAtomics::SyncQueue.new

workers = CPU_COUNT.times.map do |i|
  puts "Starting worker..."

  Ractor.new(name: "worker-#{i}") do
    puts "Starting polling..."
    while (n = QUEUE.pop) do
      puts "[#{Ractor.current.name}] #{n}"
      sleep 3
    end
  end
end

sleep 2
n = 1
loop do
  QUEUE.push(n)
  sleep 0.1
  n += 1
end
