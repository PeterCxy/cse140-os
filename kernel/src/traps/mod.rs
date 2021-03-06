mod irq;
mod trap_frame;
mod syndrome;
pub mod syscall;

use pi::interrupt::{Controller, Interrupt};

pub use self::trap_frame::TrapFrame;

use console::kprintln;
use aarch64;
use shell;
use self::syndrome::Syndrome;
use self::irq::handle_irq;
use self::syscall::handle_syscall;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern fn handle_exception(info: Info, esr: u32, tf: &mut TrapFrame) {
    if info.kind == Kind::Irq {
        if Controller::new().is_pending(Interrupt::Timer1) {
            handle_irq(Interrupt::Timer1, tf);
            return;
        }
    }
    let exception_syndrome = Syndrome::from(esr);

    if let Syndrome::Svc(num) = exception_syndrome {
        handle_syscall(num, tf);
    } else if exception_syndrome != Syndrome::WfiWfe {
        kprintln!("---- Exception ----");
        kprintln!("info: {:?}", info);
        kprintln!("syndrome: {:?}", exception_syndrome);
        kprintln!("-------------------");
        shell::shell("debug> "); // Start debug shell
    }
    tf.program_counter += 4; // Jump to the next instruction
}
