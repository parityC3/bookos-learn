use core::marker::PhantomData;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct ContextFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_addr: u32,
    pub xpsr: u32,
}

pub struct Process<'a> {
    sp: usize,
    regs: [u32; 8],
    marker: PhantomData<&'a u8>,
}

#[repr(align(8))]
pub struct AlignedStack(pub MaybeUninit<[u8; 1024]>);

extern "C" {
    fn asm_execute_process(sp: usize, regs: &mut [u32; 8]) -> usize;
}

impl<'a> Process<'a> {
    pub fn new(stack: &'a mut AlignedStack, app_main: extern "C" fn() -> !) -> Self {
        let sp = (stack.0.as_ptr() as usize) + unsafe { stack.0.assume_init_ref().len() } - 0x20;
        let context_frame: &mut ContextFrame = unsafe {
            &mut *(sp as *mut ContextFrame)
        };
        context_frame.r0 = 0;
        context_frame.r1 = 0;
        context_frame.r2 = 0;
        context_frame.r3 = 0;
        context_frame.r12 = 0;
        context_frame.lr = 0;
        context_frame.return_addr = app_main as u32;
        context_frame.xpsr = 0x0100_0000;

        Process {
            sp,
            regs: [0; 8],
            marker: PhantomData,
        }
    }

    pub fn exec(&mut self) {
        self.sp = unsafe { asm_execute_process(self.sp, &mut self.regs) }
    }
}