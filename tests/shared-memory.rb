o = Object.new
Ractor.make_shareable(o)
id = o.object_id
puts "[MAIN] #{id} #{o}"

r = Ractor.new(id) do |id|
    o2 = ObjectSpace._id2ref(id)
    puts "[NON-MAIN] #{id} #{o2}"
    Ractor.yield :done
end

r.take
