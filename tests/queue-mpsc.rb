# Multiple producers, single consumer

require_relative './helper'

QUEUE = CAtomics::SyncQueue.new

producers = 1.upto(CPU_COUNT).map do |i|
  Ractor.new(i) do |i|
    sleep 0.1 * i
    QUEUE.push(i)
    Ractor.yield :done
  end
end

consumer = Ractor.new do
  p "Starting consumer"
  while i = QUEUE.pop do
    puts i
  end
  Ractor.yield :done
end

p producers.map(&:take)
QUEUE.push(nil)
p consumer.take
