use core::slice;
use std::mem::size_of;
use std::str;
use std::cell;
use windows::Win32::System::Antimalware::AMSI_ATTRIBUTE_CONTENT_ADDRESS;
use windows::Win32::System::Antimalware::AMSI_RESULT_DETECTED;
use windows::{
    core::implement,
    core::{w, Error, HSTRING, PWSTR},
    Win32::{
        Foundation::HWND,
        System::Antimalware::{
            IAmsiStream, IAntimalwareProvider, IAntimalwareProvider_Impl,
            AMSI_ATTRIBUTE_CONTENT_SIZE, AMSI_RESULT, AMSI_RESULT_CLEAN, AMSI_RESULT_NOT_DETECTED,
        },
        UI::WindowsAndMessaging::MessageBoxW,
    },
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[implement(IAntimalwareProvider)]
pub struct AMSIProvider {
    _mutable_state: cell::RefCell<usize>,
}

impl AMSIProvider {
    pub fn new() -> Self {
        Self {
            _mutable_state: cell::RefCell::new(0),
        }
    }
}

impl IAntimalwareProvider_Impl for AMSIProvider {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Scan(&self, stream: Option<&IAmsiStream>) -> windows::core::Result<AMSI_RESULT> {
        if stream.is_none() {
            return Ok(AMSI_RESULT_NOT_DETECTED);
        }

        let amsi_stream = stream.unwrap();

        // AMSI_ATTRIBUTE_CONTENT_SIZE is ULONGLONG(8byte)
        let mut size = [0_u8; 8];
        let mut retdata: u32 = 0;
        unsafe {
            if amsi_stream.GetAttribute(
                AMSI_ATTRIBUTE_CONTENT_SIZE,
                &mut size,
                &mut retdata as *mut u32
            ).is_err() {
                return Ok(AMSI_RESULT_NOT_DETECTED);
            };
        }

        let content_size = u64::from_ne_bytes(size);
        let mut content_buf = [0u8;1024];

        let mut content_addr_arr = [0u8; size_of::<usize>()];
        let mut content_addr_size:u32 = 0;
        unsafe {
            if amsi_stream.GetAttribute(
                AMSI_ATTRIBUTE_CONTENT_ADDRESS,
                &mut content_addr_arr,
                &mut content_addr_size as *mut u32
            ).is_err() {
                return Ok(AMSI_RESULT_NOT_DETECTED);
            };
        }

        let content_addr = usize::from_ne_bytes(content_addr_arr);
        let mut content = unsafe{ 
            slice::from_raw_parts(content_addr as *const u16,  content_size as usize)
        };
       

        let content_string = String::from_utf16_lossy(content);
        //unsafe {
        //    MessageBoxW(HWND(0), &HSTRING::from(content_string.clone()), w!("content.len"), Default::default());
        //};
        if content_string.to_string().contains("rikoteki") {
            return Ok(AMSI_RESULT_DETECTED);
        }

        Ok(AMSI_RESULT_CLEAN)
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn CloseSession(&self, session: u64) {}

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn DisplayName(&self) -> Result<PWSTR, Error> {
        Ok(PWSTR::from_raw(
            HSTRING::from("AMSI Provider with Rust").as_wide().as_ptr() as *mut u16,
        ))
    }
}
