require_relative './helper'

o = Object.new
ADDRESS = obj_to_address(o)
puts "[MAIN] #{ADDRESS} #{o}"

r = Ractor.new do
  o2 = address_to_obj(ADDRESS)
  puts "[NON-MAIN] #{ADDRESS} #{o2}"
  Ractor.yield :done
end

r.take
