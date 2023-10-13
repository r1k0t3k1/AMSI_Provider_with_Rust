use std::ffi::c_void;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::* };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxW };

pub const CLSID_AMSI_PROVIDER: GUID = GUID::from_u128(0x35817bc3d875e537b9f86103d91841e9);
const IID_I_CLASS_FACTORY: GUID = GUID::from_u128(0x0000000100000000C0000000000000046);
static DLL_REF_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => ()
    }
    true
}

#[no_mangle]
pub extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    let ref_count = DLL_REF_COUNT.load(Ordering::SeqCst);
    if ref_count == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

#[no_mangle]
pub extern "stdcall" fn DllGetClassObject(
    rclsid: *const GUID,
    riid:   *const GUID,
    ppv:    *mut *mut c_void, 
) -> HRESULT {
    let rclsid = &unsafe { *rclsid };
    let riid = &unsafe { *riid };
    let ppv = unsafe { &mut *ppv };

    *ppv = std::ptr::null_mut();

    if *rclsid != CLSID_AMSI_PROVIDER {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    if *riid != IID_I_CLASS_FACTORY {
        return E_UNEXPECTED;
    }

    S_OK

}

fn attach() {
    unsafe {
        MessageBoxW(
            HWND(0),
            w!("WELCOME!"),
            w!("Hello World"),
            Default::default()
        );
    };
}

fn detach() {
    unsafe {
        MessageBoxW(
            HWND(0),
            w!("GOODBYE!"),
            w!("Hello World"),
            Default::default()
        );
    };
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
