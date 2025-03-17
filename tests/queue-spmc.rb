# Single producer, multiple consumers

require_relative './helper.rb'

QUEUE = CAtomics::QueueWithMutex.new(CPU_COUNT)

1.upto(CPU_COUNT).map do |i|
  puts "Starting worker..."

  Ractor.new(name: "worker-#{i}") do
    puts "[#{Ractor.current.name}] Starting polling..."
    while (popped = QUEUE.pop) do
      puts "[#{Ractor.current.name}] #{popped}"
      sleep 3
    end
  end
end

value_to_push = 1
loop do
  QUEUE.push(value_to_push)
  sleep 0.5 # push twice a second to make workers "starve" and enter the polling loop
  value_to_push += 1
end
