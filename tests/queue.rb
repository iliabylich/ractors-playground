require_relative './helper'

QUEUE = SyncQueue.new

producers = 1.upto(CPU_COUNT).map do |i|
  Ractor.new(i) do |i|
    sleep 0.1 * i
    QUEUE.push(i)
    Ractor.yield :done
  end
end

p producers.map(&:take)

p Ractor.new { QUEUE.push(nil); Ractor.yield :done }.take

consumer = Ractor.new do
  while i = QUEUE.pop do
    puts i
  end
  Ractor.yield :done
end

p consumer.take
