# GC.stress = true

r = Ractor.new(name: 'one that requires') do
    require 'ostruct'
    Ractor.yield OpenStruct.new(created_in_main: Ractor.main?)
end

p r.take
p OpenStruct.new(created_in_main: Ractor.main?)
