import gleam/http/elli
import gleam/http/response.{type Response}
import gleam/http/request.{type Request}
import gleam/bytes_builder.{type BytesBuilder}
import gleam/io

fn parse_request(_request: Request(t)) {
  io.println("Hello world!")
}

pub fn service(request: Request(t)) -> Response(BytesBuilder) {
  parse_request(request)
  let body = bytes_builder.from_string("Hello")
  response.new(200)
  |> response.prepend_header("made-with", "Gleam")
  |> response.set_body(body)
}

pub fn main() {
  elli.become(service, on_port: 80)
}
