#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type UCHAR = ::std::os::raw::c_uchar;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type UINT = ::std::os::raw::c_uint;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type SHORT = ::std::os::raw::c_short;
pub type LONG = ::std::os::raw::c_long;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _JOYSTICK_POSITION_V2 {
    pub bDevice: BYTE,
    pub wThrottle: LONG,
    pub wRudder: LONG,
    pub wAileron: LONG,
    pub wAxisX: LONG,
    pub wAxisY: LONG,
    pub wAxisZ: LONG,
    pub wAxisXRot: LONG,
    pub wAxisYRot: LONG,
    pub wAxisZRot: LONG,
    pub wSlider: LONG,
    pub wDial: LONG,
    pub wWheel: LONG,
    pub wAxisVX: LONG,
    pub wAxisVY: LONG,
    pub wAxisVZ: LONG,
    pub wAxisVBRX: LONG,
    pub wAxisVBRY: LONG,
    pub wAxisVBRZ: LONG,
    pub lButtons: LONG,
    pub bHats: DWORD,
    pub bHatsEx1: DWORD,
    pub bHatsEx2: DWORD,
    pub bHatsEx3: DWORD,
    pub lButtonsEx1: LONG,
    pub lButtonsEx2: LONG,
    pub lButtonsEx3: LONG,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VjdStat {
    VjdStatOwned = 0,
    VjdStatFree = 1,
    VjdStatBusy = 2,
    VjdStatMissing = 3,
    VjdStatUnknown = 4,
}

extern "C" {
    pub fn GetvJoyVersion() -> SHORT;

    pub fn vJoyEnabled() -> BOOL;

    pub fn GetvJoyProductString() -> *mut std::os::raw::c_char;

    pub fn GetvJoyManufacturerString() -> *mut std::os::raw::c_char;

    pub fn GetvJoySerialNumberString() -> *mut std::os::raw::c_char;

    pub fn DriverMatch(DllVer: *mut WORD, DrvVer: *mut WORD) -> BOOL;

    pub fn GetvJoyMaxDevices(n: *mut ::std::os::raw::c_int) -> BOOL;

    pub fn GetNumberExistingVJD(n: *mut ::std::os::raw::c_int) -> BOOL;

    pub fn GetVJDButtonNumber(rID: UINT) -> ::std::os::raw::c_int;

    pub fn GetVJDDiscPovNumber(rID: UINT) -> ::std::os::raw::c_int;

    pub fn GetVJDContPovNumber(rID: UINT) -> ::std::os::raw::c_int;

    pub fn GetVJDAxisExist(rID: UINT, Axis: UINT) -> BOOL;

    pub fn GetVJDAxisMax(rID: UINT, Axis: UINT, Max: *mut LONG) -> BOOL;

    pub fn GetVJDAxisMin(rID: UINT, Axis: UINT, Min: *mut LONG) -> BOOL;

    pub fn GetVJDStatus(rID: UINT) -> VjdStat;

    pub fn isVJDExists(rID: UINT) -> BOOL;

    pub fn GetOwnerPid(rID: UINT) -> ::std::os::raw::c_int;

    pub fn AcquireVJD(rID: UINT) -> BOOL;

    pub fn RelinquishVJD(rID: UINT);

    pub fn UpdateVJD(rID: UINT, pData: PVOID) -> BOOL;

    pub fn ResetVJD(rID: UINT) -> BOOL;

    pub fn ResetAll();

    pub fn ResetButtons(rID: UINT) -> BOOL;

    pub fn ResetPovs(rID: UINT) -> BOOL;

    pub fn SetAxis(Value: LONG, rID: UINT, Axis: UINT) -> BOOL;

    pub fn SetBtn(Value: BOOL, rID: UINT, nBtn: UCHAR) -> BOOL;

    pub fn SetDiscPov(Value: ::std::os::raw::c_int, rID: UINT, nPov: UCHAR) -> BOOL;

    pub fn SetContPov(Value: DWORD, rID: UINT, nPov: UCHAR) -> BOOL;
}
