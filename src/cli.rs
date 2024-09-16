use crate::bindings::exports;
use crate::bindings::wasi::cli::environment;
use crate::bindings::wasi::cli::terminal_input::TerminalInput;
use crate::bindings::wasi::cli::terminal_output::TerminalOutput;
use crate::bindings::wasi::cli::terminal_stderr;
use crate::bindings::wasi::cli::terminal_stdin;
use crate::bindings::wasi::cli::terminal_stdout;
use crate::bindings::wasi::cli::{stderr, stdin, stdout};
use crate::Passthrough;

impl<T: ?Sized> exports::wasi::cli::environment::Guest for Passthrough<T> {
    fn get_environment() -> Vec<(String, String)> {
        environment::get_environment()
    }

    fn get_arguments() -> Vec<String> {
        environment::get_arguments()
    }

    fn initial_cwd() -> Option<String> {
        environment::initial_cwd()
    }
}

impl<T: ?Sized> exports::wasi::cli::stdin::Guest for Passthrough<T> {
    fn get_stdin() -> exports::wasi::io::streams::InputStream {
        exports::wasi::io::streams::InputStream::new(stdin::get_stdin())
    }
}

impl<T: ?Sized> exports::wasi::cli::stdout::Guest for Passthrough<T> {
    fn get_stdout() -> exports::wasi::io::streams::OutputStream {
        exports::wasi::io::streams::OutputStream::new(stdout::get_stdout())
    }
}

impl<T: ?Sized> exports::wasi::cli::stderr::Guest for Passthrough<T> {
    fn get_stderr() -> exports::wasi::io::streams::OutputStream {
        exports::wasi::io::streams::OutputStream::new(stderr::get_stderr())
    }
}

impl<T: ?Sized> exports::wasi::cli::terminal_input::Guest for Passthrough<T> {
    type TerminalInput = TerminalInput;
}

impl exports::wasi::cli::terminal_input::GuestTerminalInput for TerminalInput {}

impl<T: ?Sized> exports::wasi::cli::terminal_output::Guest for Passthrough<T> {
    type TerminalOutput = TerminalOutput;
}

impl exports::wasi::cli::terminal_output::GuestTerminalOutput for TerminalOutput {}

impl<T: ?Sized> exports::wasi::cli::terminal_stdin::Guest for Passthrough<T> {
    fn get_terminal_stdin() -> Option<exports::wasi::cli::terminal_input::TerminalInput> {
        terminal_stdin::get_terminal_stdin()
            .map(exports::wasi::cli::terminal_input::TerminalInput::new)
    }
}

impl<T: ?Sized> exports::wasi::cli::terminal_stdout::Guest for Passthrough<T> {
    fn get_terminal_stdout() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stdout::get_terminal_stdout()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}

impl<T: ?Sized> exports::wasi::cli::terminal_stderr::Guest for Passthrough<T> {
    fn get_terminal_stderr() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stderr::get_terminal_stderr()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}
