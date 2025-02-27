require_relative './helper'

slow = CAtomics::SlowObject.new(42)
puts "Slow object has been created"

r = Ractor.new(slow) do |slow|
  5.times { slow.slow_op }
  Ractor.yield :done
end
5.times { GC.start; sleep 0.1 }
p r.take
