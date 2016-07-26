extern crate capnp;

use std::result;
use std::io;
use std::string;
use std::sync::mpsc;

use ports::IP;
use scheduler::CompMsg;

pub type Result<T> = result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Capnp(err: capnp::Error) {
            cause(err)
            display("Capnp error: {}", err)
            from()
        }
        CapnpNIS(err: capnp::NotInSchema) {
            cause(err)
            display("Capnp Not in schema: {}", err)
            from()
        }
        IO(err: io::Error) {
            cause(err)
            display("IO error: {}", err)
            from()
        }
        FromUtf8(err: string::FromUtf8Error) {
            cause(err)
            display("From utf8 error: {}", err)
            from()
        }
        Mpsc(err: mpsc::RecvError) {
            cause(err)
            display("Mpsc error: {}", err)
            from()
        }
        MpscTryRecv(err: mpsc::TryRecvError) {
            cause(err)
            display("Mpsc error: {}", err)
            from()
        }
        MpscSend {
            display("Mpsc cannot send")
            from(mpsc::SendError<IP>)
            from(mpsc::SendError<CompMsg>)
        }
        Misc(err: String) {
            display("Misc: {}", err)
            from()
        }
        ComponentNotFound(name: String) {
            display("Component {} not found", name)
        }
        PortNotFound(comp: String, port: String) {
            display("Port {} not found on {}", port, comp)
        }
        SelectionNotFound(comp: String, port: String, selection: String) {
            display("Selection {} not found on the port {} of the component {}", selection, port, comp)
        }
        OutputPortNotConnected(comp: String, port: String) {
            display("The port {} of the component {} is not connected", port, comp)
        }
        ArrayOutputPortNotConnected(comp: String, port: String, selection: String) {
            display("{}() {}[{}] not connected", comp, port, selection)
        }
        CannotRemove(comp: String) {
            display("Cannot remove the component {}", comp)
        }
    }
}
