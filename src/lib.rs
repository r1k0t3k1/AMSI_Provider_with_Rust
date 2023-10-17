use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use windows::Win32::System::Com::{CoInitialize, StringFromCLSID};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::{
    RegCreateKeyExW, RegSetValueExW, HKEY, HKEY_CLASSES_ROOT, KEY_CREATE_SUB_KEY, KEY_SET_VALUE,
    REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey, HKEY_LOCAL_MACHINE,
};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::{core::*, Win32::UI::WindowsAndMessaging::MessageBoxW};
use windows::{Win32::Foundation::*, Win32::System::SystemServices::*};

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
        }
        DLL_PROCESS_DETACH => detach(),
        _ => (),
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
    riid: *const GUID,
    ppv: *mut *mut c_void,
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
    let mut lpfilename = [0u16; 300];

    let iret: u32 = unsafe { GetModuleFileNameW(*G_MODULE.get().unwrap(), &mut lpfilename) };

    if iret == 0 {
        let err = unsafe { GetLastError() };
        println!("{:?}", err);
        return;
    }

    let mut utf16_vec = lpfilename.to_vec();
    utf16_vec.retain(|&x| x != 0);
    let utf16_string = String::from_utf16(&utf16_vec).unwrap();

    unsafe {
        MessageBoxW(
            HWND(0),
            &HSTRING::from(utf16_string.clone()),
            w!("DLL path"),
            Default::default(),
        );
    };

    let CLSID = unsafe {
        StringFromCLSID(&CLSID_AMSI_PROVIDER as *const GUID)
            .unwrap()
            .to_string()
            .unwrap()
    };

    unsafe {
        MessageBoxW(
            HWND(0),
            &HSTRING::from(CLSID.clone()),
            w!("CLSID"),
            Default::default(),
        );
    };

    let szRegKey = format!("{}{}", String::from("CLSID\\"), &CLSID);

    let mut clsid_phkresult = HKEY::default();

    unsafe {
        let result = RegCreateKeyExW(
            HKEY_CLASSES_ROOT,
            &HSTRING::from(szRegKey),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE | KEY_CREATE_SUB_KEY,
            None,
            &mut clsid_phkresult as *mut HKEY,
            None,
        );
    }
    println!("{:?}", clsid_phkresult);

    let mut Description: Vec<u8> = vec![];
    for c in "AMSI Provider with Rust".as_bytes().iter() {
        Description.push(*c);
        Description.push(0);
    }

    unsafe {
        let result = RegSetValueExW(
            clsid_phkresult,
            PCWSTR::null(),
            0,
            REG_SZ,
            Some(&Description),
        );
        if result.is_err() {
            MessageBoxW(
                HWND(0),
                w!("Failed to set value"),
                w!("CLSID"),
                Default::default(),
            );
        }
    }

    // Create InProcServer32 sub key.
    let mut inprocserver_phkresult = HKEY::default();
    unsafe {
        let result = RegCreateKeyExW(
            clsid_phkresult,
            &HSTRING::from("InProcServer32"),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &mut inprocserver_phkresult as *mut HKEY,
            None,
        );
        if result.is_err() {
            MessageBoxW(
                HWND(0),
                w!("Failed to Create InProcServer32 sub key."),
                w!("CLSID"),
                Default::default(),
            );
        }
    }

    // Set module file name to InProcServer32.
    let mut module_file_name: Vec<u8> = vec![];
    for c in utf16_string.as_bytes().iter() {
        module_file_name.push(*c);
        module_file_name.push(0);
    }

    unsafe {
        let result = RegSetValueExW(
            inprocserver_phkresult,
            PCWSTR::null(),
            0,
            REG_SZ,
            Some(&module_file_name),
        );
        if result.is_err() {
            MessageBoxW(
                HWND(0),
                w!("Failed to set module file name to InProcServer32"),
                w!("CLSID"),
                Default::default(),
            );
        }
    }

    // Set Both to InProcServer32.
    let mut both: Vec<u8> = vec![];
    for c in "Both".as_bytes().iter() {
        both.push(*c);
        both.push(0);
    }

    unsafe {
        let result = RegSetValueExW(
            inprocserver_phkresult,
            w!("ThreadingModel"),
            0,
            REG_SZ,
            Some(&both),
        );
        if result.is_err() {
            MessageBoxW(
                HWND(0),
                w!("Failed to set both to InProcServer32"),
                w!("CLSID"),
                Default::default(),
            );
        }
    }

    unsafe { 
        RegCloseKey(inprocserver_phkresult);
        RegCloseKey(clsid_phkresult);
    }

    let szAMSIProvider = format!("Software\\Microsoft\\AMSI\\Providers\\{}", &CLSID);
    println!("{}", szAMSIProvider);
    // Create InProcServer32 sub key.
    let mut amsiprovider_phkresult = HKEY::default();
    unsafe {
        let result = RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            &HSTRING::from(szAMSIProvider),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE | KEY_CREATE_SUB_KEY,
            None,
            &mut amsiprovider_phkresult as *mut HKEY,
            None,
        );
        if result.is_err() {
            MessageBoxW(
                HWND(0),
                w!("Failed to Create AMSI Provider key."),
                w!("CLSID"),
                Default::default(),
            );
        }
    }
}

fn attach() {
    unsafe {
        MessageBoxW(
            HWND(0),
            w!("WELCOME!"),
            w!("Hello World"),
            Default::default(),
        );
    };
}

fn detach() {
    unsafe {
        MessageBoxW(
            HWND(0),
            w!("GOODBYE!"),
            w!("Hello World"),
            Default::default(),
        );
    };
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
