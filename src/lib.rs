use windows::{ Win32::Foundation::*, Win32::System::SystemServices::* };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA };

#[no_mangle]
#[allow(no_snake_case, unused_variables)]
extern "system" fn Dllmain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => ()
    }

    true
}

fn attach() {
    unsafe {
        MessageBoxA(
            HWND(0),
            s!("WELCOME!"),
            s!("Hello World"),
            Default::default()
        );
    };
}

fn detach() {
    unsafe {
        MessageBoxA(
            HWND(0),
            s!("GOODBYE!"),
            s!("Hello World"),
            Default::default()
        );
    };
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
