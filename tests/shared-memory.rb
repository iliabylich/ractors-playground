o = Object.new
Ractor.make_shareable(o)
ID = o.object_id
puts "[MAIN] #{ID} #{o}"

r = Ractor.new do
  o2 = ObjectSpace._id2ref(ID)
  puts "[NON-MAIN] #{ID} #{o2}"
  Ractor.yield :done
end

r.take
