use {
    pinocchio::{log::sol_log, program_error::ProgramError},
    solana_program_option::COption,
};

pub type Result<T> = core::result::Result<T, ProgramError>;

// #[inline(always)]
// pub fn serialize<T>(data: &T) -> &[u8]
// where
//     T: Pod + Zeroable,
// {
//     bytes_of(data)
// }

// #[inline(always)]
// pub fn serialize_mut<T>(data: &mut T) -> &mut [u8]
// where
//     T: Pod + Zeroable,
// {
//     bytes_of_mut(data)
// }

// #[inline(always)]
// pub fn deserialize<T>(data: &[u8]) -> Result<&T>
// where
//     T: Pod + Zeroable,
// {
//     try_from_bytes(data).map_err(|_| ProgramError::InvalidAccountData)
// }

// #[inline(always)]
// pub unsafe fn deserialize_unchecked<T: Pod>(data: &[u8]) -> &T {
//     &*(data.as_ptr() as *const T)
// }

// #[inline(always)]
// pub fn deserialize_mut<T>(data: &mut [u8]) -> Result<&mut T>
// where
//     T: Pod + Zeroable,
// {
//     try_from_bytes_mut(data).map_err(|_| ProgramError::InvalidAccountData)
// }

// #[inline(always)]
// pub unsafe fn deserialize_mut_unchecked<T: Pod>(data: &mut [u8]) -> &mut T {
//     &mut *(data.as_mut_ptr() as *mut T)
// }

#[inline(always)]
pub fn from_c_option<T>(data: COption<T>) -> Option<T> {
    if data.is_some() {
        Some(data.unwrap())
    } else {
        None
    }
}

pub fn show<T: core::fmt::Debug>(label: &str, data: T) {
    sol_log(&format!("✅ {}: {:?}", label, data));
}
