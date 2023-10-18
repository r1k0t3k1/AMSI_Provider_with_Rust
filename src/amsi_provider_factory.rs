use std::ffi::c_void;
use std::{ptr, mem};

use windows::Win32::System::Antimalware::IAntimalwareProvider;
use windows::core::{implement, Error, IUnknown, GUID};
use windows::Win32::Foundation::{BOOL, E_NOTIMPL, E_POINTER, E_INVALIDARG, CLASS_E_NOAGGREGATION, E_NOINTERFACE};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::core::ComInterface;

#[implement(IClassFactory)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AMSIProviderFactory;

impl IClassFactory_Impl for AMSIProviderFactory {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvovject: *mut *mut c_void,
    ) -> windows::core::Result<()> {
        if ppvovject.is_null() {
            return Err(E_POINTER.into());
        }

        unsafe { *ppvovject = ptr::null_mut() };

        if riid.is_null() {
            return Err(E_INVALIDARG.into());
        }
        let riid = unsafe { *riid };

        if punkouter.is_some() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }

        if riid != IAntimalwareProvider::IID {
            return Err(E_NOINTERFACE.into());
        }

        let amsi_provider: IAntimalwareProvider = crate::amsi_provider::AMSIProvider::new().into();
        unsafe { *ppvovject = mem::transmute(amsi_provider) };
        Ok(())
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}
