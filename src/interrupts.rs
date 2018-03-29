use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;


lazy_static!{
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn load_idt() {
    IDT.load();
}



type HandlerFunc = extern "x86-interrupt" fn(_: &mut ExceptionStackFrame);


extern "x86-interrupt" fn breakpoint_handler(st_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION");
    println!("{:?}", st_frame);
}


