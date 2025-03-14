require_relative './helper.rb'
require 'socket'
require 'webrick'

QUEUE = CAtomics::MpmcQueue.new(16)
# GC.disable

def log(s)
  $stderr.puts "[#{Ractor.current.name}] #{s}"
end

def read_body(conn)
  body = ""
  buf = String.new(capacity: 1024)
  started_at = now
  loop do
    buf = conn.read_nonblock(1024)
    body += buf
    if buf.bytesize < 1024
      break
    end
  rescue IO::EAGAINWaitReadable
    if now > started_at + 1
      raise 'timeout error'
    end
    conn.wait_readable(0.1)
  rescue EOFError
    break
  end
  if body.empty?
    raise "no data received"
  end
  body
end

def parse_body(body)
  start_line, rest = body.split("\r\n", 2)
  http_method, path, protocol = start_line.split(' ')
  if protocol != "HTTP/1.1" && protocol != "HTTP/1.0"
    raise "only HTTP/1.* is supported"
  end
  headers, body = rest.split("\r\n\r\n", 2)
  headers = headers.split("\r\n").map { |header| header.split(": ", 2) }.to_h
  [http_method, path, protocol, headers, body]
end

STATUS_CODE_TO_STATUS_NAME = Ractor.make_shareable(WEBrick::HTTPStatus::StatusMessage.dup)

def reply(conn, status, headers, body)
  res = ""
  res << "HTTP/1.1 #{status} #{STATUS_CODE_TO_STATUS_NAME[status]}\r\n"
  headers = headers.merge('Content-Length' => body.bytesize)
  headers.each do |name, value|
    res << "#{name}: #{value}\r\n"
  end
  res << "\r\n"
  res << body
  conn.write(res)
end

def process_request(conn)
  body = read_body(conn)
  http_method, path, protocol, headers, body = parse_body(body)

  log "#{http_method} #{path}"

  case [http_method, path]
  when ["GET", "/slow"]
    heavy_computation(100)
    reply(conn, 200, {}, "the endpoint is slow (100ms)")
  when ["GET", "/fast"]
    reply(conn, 200, {}, "yes, it's fast")
  else
    reply(conn, 404, {}, "Unknown path #{path}")
  end
rescue Exception => e
  log e.class.name + " " + e.message + " " + e.backtrace.join("\n    ")

  reply(conn, 500, {}, "Internal server error")
ensure
  conn.close
end

workers = 1.upto(CPU_COUNT).map do |i|
  puts "Starting worker-#{i}..."

  Ractor.new(name: "worker-#{i}") do
    while (conn = QUEUE.pop) do
      process_request(conn)
    end
    log "exiting..."
    Ractor.yield :done
  rescue Exception => e
    log e.class.name + " " + e.message + " " + e.backtrace.join("\n    ")
    Ractor.yield :crashed
  end
end

trap("SIGINT") do
  puts "Exiting..."
  CPU_COUNT.times { QUEUE.push(nil) }
  p workers.map(&:take)
  exit(0)
end

puts "Starting server..."

server = Socket.tcp_server_loop(8080) do |conn, addr|
  # puts "Got connection, forwarding to a worker..."

  if ENV['SEQ']
    process_request(conn)
  else
    QUEUE.push(conn)
  end
end
