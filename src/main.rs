#![no_main]
#![no_std]
#![feature(naked_functions)]

use core::arch::asm;
use core::ptr;
use core::panic::PanicInfo;
use core::mem::MaybeUninit;
use cortex_m_semihosting::hprintln;

mod systick;
mod process;
use process::ContextFrame;

#[repr(align(8))]
struct AlignedStack(MaybeUninit<[u8; 1024]>);

const CFSR_ADDR: usize = 0xE000_ED28;
const SHCSR_ADDR: usize = 0xE000_ED24;
const HFSR_ADDR: usize = 0xE000_ED2C;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    hprintln!("Hello World").unwrap();

    systick::init();

    #[link_section = ".app_stack"]
    static mut APP_STACK: AlignedStack = AlignedStack(MaybeUninit::uninit());
    hprintln!("APP_STACK ptr:{:p}", &APP_STACK).unwrap();
    let ptr = (APP_STACK.0.as_ptr() as *const u8 as usize) + 1024 - 0x20;
    let context_frame: &mut ContextFrame = &mut *(ptr as *mut ContextFrame);
    hprintln!("context_frame ptr:{:p}", context_frame).unwrap();
    context_frame.r0 = 0;
    context_frame.r1 = 0;
    context_frame.r2 = 0;
    context_frame.r3 = 0;
    context_frame.r12 = 0;
    context_frame.lr = 0;
    context_frame.return_addr = app_main as u32;
    context_frame.xpsr = 0x0100_0000;

    hprintln!("CFSR:{:X}", ptr::read_volatile(CFSR_ADDR as *mut u32)).unwrap();
    hprintln!("SHCSR:{:X}", ptr::read_volatile(SHCSR_ADDR as *mut u32)).unwrap();
    hprintln!("HFSR:{:X}", ptr::read_volatile(HFSR_ADDR as *mut u32)).unwrap();
    asm_execute_process(ptr);

    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    hprintln!("DefaultException").unwrap();
    loop {}
}
#[no_mangle]
pub extern "C" fn NMI() {
    hprintln!("NMI").unwrap();
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn HardFault() {
    hprintln!("HardFault").unwrap();
    hprintln!("CFSR:{:X}", ptr::read_volatile(CFSR_ADDR as *mut u32)).unwrap();
    hprintln!("SHCSR:{:X}", ptr::read_volatile(SHCSR_ADDR as *mut u32)).unwrap();
    hprintln!("HFSR:{:X}", ptr::read_volatile(HFSR_ADDR as *mut u32)).unwrap();
    loop {}
}
#[no_mangle]
pub extern "C" fn MemManage() {
    hprintln!("MemManage").unwrap();
    loop {}
}
#[no_mangle]
pub extern "C" fn BusFault() {
    hprintln!("BusFault").unwrap();
    loop {}
}
#[no_mangle]
pub extern "C" fn UsageFault() {
    hprintln!("UsageFault").unwrap();
    loop {}
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn SVCall() {
    asm!(
        "cmp lr, #0xfffffff9",
        "bne 1f",

        "mov r0, #1",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfffd",
        "movt lr, #0xffff",
        "bx lr",

        "1:",
        "mov r0, #0",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfff9",
        "movt lr, #0xffff",
        "bx lr",
        options(noreturn),
    );
}

#[no_mangle]
pub extern "C" fn PendSV() {
    hprintln!("PendSV").unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    hprintln!("Sysick").unwrap();
}
extern "C" fn app_main() -> ! {
    hprintln!("App").unwrap();
    unsafe { asm!("svc 0"); }
    loop {}
}

extern "C" {
    fn asm_execute_process(sp: usize);
}
