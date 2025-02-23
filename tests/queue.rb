require_relative './helper'

QUEUE = SyncQueue.new

producers = 1.upto(CPU_COUNT).map do |i|
  Ractor.new(i) do |i|
    sleep 0.1 * i
    QUEUE.push(i)
    Ractor.yield :done
  end
end

producers << Ractor.new { sleep 2; QUEUE.push(nil); Ractor.yield :done }

consumer = Ractor.new do
  while i = QUEUE.pop do
    puts i
  end
  Ractor.yield :done
end

p [*producers, consumer].map(&:take)
