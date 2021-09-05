// Based on Capnproto Rust (github.com/capnproto/capnproto-rust), licenced under the MIT License.
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

pub mod schema_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema_capnp.rs"));
}

mod codegen;
mod codegen_types;
mod pointer_constants;

pub(crate) fn convert_io_err(err: std::io::Error) -> capnp::Error {
    use std::io;
    let kind = match err.kind() {
        io::ErrorKind::TimedOut => capnp::ErrorKind::Overloaded,
        io::ErrorKind::BrokenPipe
        | io::ErrorKind::ConnectionRefused
        | io::ErrorKind::ConnectionReset
        | io::ErrorKind::ConnectionAborted
        | io::ErrorKind::NotConnected => capnp::ErrorKind::Disconnected,
        _ => capnp::ErrorKind::Failed,
    };
    capnp::Error {
        description: format!("{}", err),
        kind: kind,
    }
}

pub fn main() {
    //! Generates Rust code according to a `schema_capnp::code_generator_request` read from stdin.

    codegen::CodeGenerationCommand::new()
        .output_directory(::std::path::Path::new("."))
        .run(::std::io::stdin())
        .expect("failed to generate code");
}
