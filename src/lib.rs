use std::ffi::c_void;
use std::sync::atomic::{ AtomicUsize, Ordering };
use windows::{ Win32::Foundation::*, Win32::System::SystemServices::* };
use windows::Win32::System::Com::{ CoInitialize, StringFromCLSID };
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxW };
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::System::Registry::{RegCreateKeyExW, HKEY_CLASSES_ROOT, REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, KEY_CREATE_SUB_KEY, HKEY};
use std::sync::OnceLock;

static G_MODULE: OnceLock<HMODULE> = OnceLock::new();


const CLSID_AMSI_PROVIDER: GUID = GUID::from_u128(0x35817bc3d875e537b9f86103d91841e9);
const IID_I_CLASS_FACTORY: GUID = GUID::from_u128(0x0000000100000000C0000000000000046);

static DLL_REF_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            attach();
            G_MODULE.set(dll_module).unwrap();
            DllRegisterServer();
        },
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

#[no_mangle]
pub extern "stdcall" fn DllRegisterServer() {
   let _ = unsafe { CoInitialize(None) }; 
   let mut lpfilename = [0u16;300];

   let iret: u32 = unsafe{ GetModuleFileNameW(*G_MODULE.get().unwrap(), &mut lpfilename) };

   if iret == 0 {
     let err = unsafe { GetLastError() };
     println!("{:?}", err);
     return;
   }

   let mut utf16_vec = lpfilename.to_vec();
   utf16_vec.retain(|&x| x != 0);
   let utf16_string = String::from_utf16(&utf16_vec).unwrap();

   unsafe {
    MessageBoxW(HWND(0), &HSTRING::from(utf16_string), w!("DLL path"), Default::default());
   };

   let CLSID = unsafe{ StringFromCLSID(&CLSID_AMSI_PROVIDER as *const GUID).unwrap().to_string().unwrap() };

   unsafe {
    MessageBoxW(HWND(0), &HSTRING::from(CLSID.clone()), w!("CLSID"), Default::default());
   };

   let szRegKey = format!("{}{}", String::from("CLSID\\"), &CLSID);
   let szAMSIProvider = format!("Software\\Microsoft\\AMSI\\Providers\\{:?}", CLSID);

   let mut phkresult = HKEY::default();

   unsafe {
     let result = RegCreateKeyExW(
       HKEY_CLASSES_ROOT,
       &HSTRING::from(szRegKey),
       0,
       PCWSTR::null(),
       REG_OPTION_NON_VOLATILE,
       KEY_SET_VALUE | KEY_CREATE_SUB_KEY,
       None,
       &mut phkresult as *mut HKEY,
       None,
     );
   }
   println!("{:?}", phkresult);
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
