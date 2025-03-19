require_relative './helper'

array = []
ADDRESS = obj_to_address(array)

ractors = 2.times.map do
  Ractor.new do
    obj = address_to_obj(ADDRESS)
    1_000_000.times do
      obj.push(42)
      obj.pop
    end
    Ractor.yield :done
  end
end

p ractors.map(&:take)
p array.size
