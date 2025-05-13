use windows::Win32::{
    Foundation::{
        HANDLE,
        HMODULE
    },
    UI::WindowsAndMessaging,
    System::{
      Threading,
      Diagnostics::Debug,
      ProcessStatus
    }
};

use std::io::{ Error, ErrorKind };
use windows_core::PCSTR;
use std::ffi::CString;
use core::ffi::c_void;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Process {
    p_handle: HANDLE,
    pub modules: BTreeMap<String, ProcessStatus::MODULEINFO>
}

impl Process {
    fn get_modules(p_handle: HANDLE) -> Result<BTreeMap<String, ProcessStatus::MODULEINFO>, Error> {
        unsafe {
            let mut map: BTreeMap<String, ProcessStatus::MODULEINFO> = BTreeMap::new();
            let mut h_modules: Vec<HMODULE> = vec![HMODULE(0 as *mut c_void); 1024];
            let mut bytes_needed: u32 = 0;

            ProcessStatus::EnumProcessModules(
                p_handle,
                h_modules.as_mut_ptr(),
                (std::mem::size_of::<HMODULE>() * h_modules.len()) as u32,
                &mut bytes_needed
            ).unwrap();

            let module_count = bytes_needed as usize / std::mem::size_of::<HMODULE>();
            h_modules.truncate(module_count);

            for module in h_modules.iter() {
                let mut module_name = vec![0u8; 260];

                let length = ProcessStatus::GetModuleFileNameExA(
                    Some(p_handle),
                    Some(*module),
                    &mut module_name
                );

                module_name.truncate(length as usize);

                let full_string = String::from_utf8_lossy(&module_name).to_string();

                let dll_name = full_string.rsplit('\\').next().unwrap().to_string();

                let mut mod_info: ProcessStatus::MODULEINFO = std::mem::zeroed();

                ProcessStatus::GetModuleInformation(
                    p_handle,
                    *module,
                    &mut mod_info,
                    std::mem::size_of::<ProcessStatus::MODULEINFO>() as u32,
                );

                map.insert(dll_name, mod_info);
            }

            Ok(map)
        }
    }
    pub fn new(input: &str) -> Result<Self, Error> {
        unsafe {
            let window_name = CString::new(input).unwrap();

            let hwnd = WindowsAndMessaging::FindWindowA(None, PCSTR::from_raw(window_name.as_ptr() as _)).unwrap();

            let mut process_id: u32 = 0;
            let _ = WindowsAndMessaging::GetWindowThreadProcessId(hwnd, Some(&mut process_id));

            let p_handle = Threading::OpenProcess(Threading::PROCESS_ALL_ACCESS, false, process_id).unwrap();

            if p_handle.is_invalid() {
                return Err(Error::new(ErrorKind::Other, "Process handle is invald!"));
            }

            let map = Self::get_modules(p_handle)?;

            Ok(Self {
                p_handle,
                modules: map
            })
        }
    }

    pub fn read<T>(&self, address: usize) -> Result<T, Error> {
        unsafe {
            let mut buffer: T = std::mem::zeroed();
            let mut bytes_read: usize = 0;

            Debug::ReadProcessMemory(
                self.p_handle,
                address as *const c_void,
                &mut buffer as *mut _ as *mut c_void,
                std::mem::size_of::<T>(),
                Some(&mut bytes_read)
            )?;

            Ok(buffer)
        }
    }

    pub fn read_buffer(&self, address: usize, size: usize) -> Result<Vec<u8>, Error> {
        unsafe {

            let mut buffer = vec![0u8; size];
            let mut bytes_read: usize = 0;

            Debug::ReadProcessMemory(
                self.p_handle,
                address as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                size,
                Some(&mut bytes_read)
            )?;

            Ok(buffer)
        }
    }

    pub fn write<T>(&self, address: usize, value: T) -> Result<(), Error> {
        unsafe {
            let mut bytes_written: usize = 0;

            Debug::WriteProcessMemory(
                self.p_handle,
                address as *const c_void,
                &value as *const _ as *const c_void,
                std::mem::size_of::<T>(),
                Some(&mut bytes_written),
            )?;

            Ok(())
        }
    }
}
