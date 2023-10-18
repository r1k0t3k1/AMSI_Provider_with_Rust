use windows::{
    core::{w, Error, HSTRING, PWSTR},
    Win32::System::Antimalware::{
        IAmsiStream, IAntimalwareProvider_Impl, AMSI_RESULT, AMSI_RESULT_CLEAN,
    },
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AMSI_Provider {}

impl IAntimalwareProvider_Impl for AMSI_Provider {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Scan(&self, stream: Option<&IAmsiStream>) -> Result<AMSI_RESULT, Error> {
        Ok(AMSI_RESULT_CLEAN)
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn CloseSession(&self, session: u64) {}

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn DisplayName(&self) -> Result<PWSTR, Error> {
        Ok(PWSTR::from_raw(
            HSTRING::from("AMSI Provider with Rust").as_wide().as_ptr() as *mut u16
        ))
    }
}
