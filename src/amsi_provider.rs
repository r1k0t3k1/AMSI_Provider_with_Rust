use std::cell;

use windows::{
    core::implement,
    core::{w, Error, HSTRING, PWSTR},
    Win32::{System::Antimalware::{
        IAmsiStream, IAntimalwareProvider, IAntimalwareProvider_Impl, AMSI_RESULT,
        AMSI_RESULT_CLEAN,
    }, UI::WindowsAndMessaging::MessageBoxW, Foundation::HWND},
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
    fn Scan(&self, stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT, Error> {
    	unsafe {
    	    MessageBoxW(
    	        HWND(0),
    	        w!("CLEAN!"),
    	        w!("Scanning..."),
    	        Default::default(),
    	    );
    	};
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
