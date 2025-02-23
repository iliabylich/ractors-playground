require_relative './helper'

QUEUE = SyncQueue.new

producer = Ractor.new do
  1.upto(5) do |i|
    QUEUE.push(i)
  end
  QUEUE.push(nil)
  Ractor.yield :done
end

consumer = Ractor.new do
  while i = QUEUE.pop do
    puts i
  end
  Ractor.yield :done
end

p [producer, consumer].map(&:take)
