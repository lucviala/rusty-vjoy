//latest sdk version  2.1.8.33
//latest driver version 2.1.9.1
//what do?

mod ffi;

pub type VJDStat = ffi::VjdStat;
pub type JoystickPosition = ffi::_JOYSTICK_POSITION_V2;

/// HID Descriptor definitions - Axes
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HidUsage {
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    RX = 0x33,
    RY = 0x34,
    RZ = 0x35,
    SL0 = 0x36,
    SL1 = 0x37,
    WHL = 0x38,
    POV = 0x39,
}

/// Return the version number of the installed vJoy.
///
/// To be used only after vJoyEnabled()
pub fn get_vjoy_version() -> i16 {
    unsafe { ffi::GetvJoyVersion() }
}

/// Returns TRUE if vJoy version 2.x is installed and enabled.
pub fn vjoy_enabled() -> bool {
    unsafe { ffi::vJoyEnabled() != 0 }
}

pub fn get_vjoy_product_string() -> String {
    //must be a better way to do that
    unsafe {
        let ptr = ffi::GetvJoyProductString();

        let mut string = String::with_capacity(23);

        for i in 0..46 {
            let ptr = ptr.add(i);

            let c_str = std::ffi::CStr::from_ptr(ptr as *mut i8);

            string.push_str(c_str.to_str().unwrap());
        }

        string
    }
}

pub fn get_vjoy_manufacturer_string() -> String {
    //must be a better way to do that
    unsafe {
        let ptr = ffi::GetvJoyManufacturerString();

        let mut string = String::with_capacity(16);

        for i in 0..32 {
            let ptr = ptr.add(i);

            let c_str = std::ffi::CStr::from_ptr(ptr);

            string.push_str(c_str.to_str().unwrap());
        }

        string
    }
}

pub fn get_vjoy_serial_number_string() -> String {
    //must be a better way to do that
    unsafe {
        let ptr = ffi::GetvJoySerialNumberString();

        let mut string = String::with_capacity(5);

        for i in 0..10 {
            let ptr = ptr.add(i);

            let c_str = std::ffi::CStr::from_ptr(ptr as *mut i8);

            string.push_str(c_str.to_str().unwrap());
        }

        string
    }
}

/// Returns TRUE if vJoyInterface.dll file version and vJoy Driver version are identical. Otherwise returns FALSE.
///
/// Optional (You may pass NULL):
///
/// Output parameter DllVer: If a pointer to WORD is passed then the value of the DLL file version will be written to this parameter (e.g. 0x215).
///
/// Output parameter DrvVer: If a pointer to WORD is passed then the value of the Driver version will be written to this parameter (e.g. 0x215).
pub fn driver_match() -> (bool, u16, u16) {
    let mut dll_ver = 0;
    let mut driver_ver = 0;

    unsafe {
        let status = ffi::DriverMatch(&mut dll_ver, &mut driver_ver) != 0;

        (status, dll_ver, driver_ver)
    }
}

/// What is the maximum possible number of vJoy devices
pub fn get_vjoy_max_devices() -> i32 {
    let mut number = 0;

    unsafe {
        if ffi::GetvJoyMaxDevices(&mut number) != 0 {
            number
        } else {
            0
        }
    }
}

/// What is the number of vJoy devices currently enabled.
pub fn get_number_existing_vjd() -> i32 {
    let mut number = 0;

    unsafe {
        if ffi::GetNumberExistingVJD(&mut number) != 0 {
            number
        } else {
            0
        }
    }
}

/// If function succeeds, returns the number of buttons in the specified device. Valid values are 0 to 128
///
/// If function fails, returns a negative error code:
///
/// • NO_HANDLE_BY_INDEX
///
/// • BAD_PREPARSED_DATA
///
/// • NO_CAPS
///
/// • BAD_N_BTN_CAPS
///
/// • BAD_BTN_CAPS
///
/// • BAD_BTN_RANGE
pub fn get_vjd_button_number(id: u32) -> i32 {
    unsafe { ffi::GetVJDButtonNumber(id) }
}

/// Returns the number of discrete-type POV hats in the specified device.
///
/// Discrete-type POV Hat values may be North, East, South, West or neutral.
///
/// Valid values are 0 to 4 (from version 2.0.1)
pub fn get_vjd_disc_pov_number(id: u32) -> i32 {
    unsafe { ffi::GetVJDDiscPovNumber(id) }
}

/// Returns the number of continuous-type POV hats in the specified device.
///
/// Continuous-type POV Hat values may be 0 to 35900.
///
/// Valid values are 0 to 4 ( from version 2.0.1)
pub fn get_vjd_cont_pov_number(id: u32) -> i32 {
    unsafe { ffi::GetVJDContPovNumber(id) }
}

/// Returns TRUE if the specified axis exists in the specified device.
pub fn get_vjd_axis_exist(id: u32, usage: HidUsage) -> bool {
    unsafe { ffi::GetVJDAxisExist(id, usage as u32) != 0 }
}

pub fn get_vjd_axis_max(id: u32, usage: HidUsage) -> i32 {
    let mut max = 0;

    unsafe {
        if ffi::GetVJDAxisMax(id, usage as u32, &mut max) != 0 {
            max
        } else {
            0
        }
    }
}

pub fn get_vjd_axis_min(id: u32, usage: HidUsage) -> i32 {
    let mut min = 0;

    unsafe {
        if ffi::GetVJDAxisMin(id, usage as u32, &mut min) != 0 {
            min
        } else {
            0
        }
    }
}

/// Returns the status of the specified device
///
/// The status can be one of the following values:
///
/// • VJD_STAT_OWN // The vJoy Device is owned by this application.
///
/// • VJD_STAT_FREE // The vJoy Device is NOT owned by any application (including this one).
///
/// • VJD_STAT_BUSY // The vJoy Device is owned by another application.
///
///     It cannot be acquired by this application.
///
/// • VJD_STAT_MISS // The vJoy Device is missing. It either does not exist or the driver is disabled.
///
/// • VJD_STAT_UNKN // Unknown
pub fn get_vjd_status(id: u32) -> VJDStat {
    unsafe { ffi::GetVJDStatus(id) }
}

/// Returns TRUE if the specified device exists (Configured and enabled).
///
/// Returns FALSE otherwise (Including the following cases: Device does not exist, disabled, driver not installed)
pub fn is_vjd_exists(id: u32) -> bool {
    unsafe { ffi::isVJDExists(id) != 0 }
}

/// Returns the Process ID (PID) of the process that owns the specified device.
///
/// If the device is owned by a process, then the function returns a positive integer which is the PID of the owner.
///
/// Otherwise, the function returns one of the following negative numbers:
///
/// NO_FILE_EXIST (-13): Usually indicates a FREE device (No owner)
///
/// NO_DEV_EXIST (-12): Usually indicates a MISSING device
///
/// BAD_DEV_STAT (-11): Indicates some internal problem
pub fn get_owner_pid(id: u32) -> i32 {
    unsafe { ffi::GetOwnerPid(id) }
}

/// Acquire the specified device.
///
/// Only a device in state VJD_STAT_FREE can be acquired.
///
/// If acquisition is successful the function returns TRUE and the device status becomes VJD_STAT_OWN.
pub fn acquire_vjd(id: u32) -> bool {
    unsafe { ffi::AcquireVJD(id) != 0 }
}

/// Relinquish the previously acquired specified device.
///
/// Use only when device is state VJD_STAT_OWN.
///
/// State becomes VJD_STAT_FREE immediately after this function returns.
pub fn relinquish_vjd(id: u32) {
    unsafe {
        ffi::RelinquishVJD(id);
    }
}

/// Update the position data of the specified device.
///
/// Use only after device has been successfully acquired.
///
/// Input parameter is a pointer to structure of type JOYSTICK_POSITION that holds the position data.
///
/// Returns TRUE if device updated.
pub fn update_vjd(id: u32, data: *mut JoystickPosition) {
    unsafe {
        ffi::UpdateVJD(id, data as *mut std::ffi::c_void);
    }
}

/// Resets all the controls of the specified device to a set of values.
///
/// These values are hard coded in the interface DLL and are currently set as follows:
///
/// • Axes X, Y & Z: Middle point.
///
/// • All other axes:0.
///
/// • POV Switches: Neutral (-1).
///
/// • Buttons: Not Pressed (0).
pub fn reset_vjd(id: u32) {
    unsafe {
        ffi::ResetVJD(id);
    }
}

/// Resets all the controls of the all devices to a set of values.
///
/// See function Reset VJD for details.
pub fn reset_all() {
    unsafe {
        ffi::ResetAll();
    }
}

/// Resets all buttons (To 0) in the specified device.
pub fn reset_buttons(id: u32) {
    unsafe {
        ffi::ResetButtons(id);
    }
}

/// Resets all POV Switches (To -1) in the specified device.
pub fn reset_povs(id: u32) {
    unsafe {
        ffi::ResetPovs(id);
    }
}

/// Write Value to a given axis defined in the specified VDJ.
///
/// Value in the range 0x1-0x8000
pub fn set_axis(value: i32, id: u32, axis: HidUsage) {
    unsafe {
        ffi::SetAxis(value, id, axis as u32);
    }
}

/// Write Value (TRUE or FALSE) to a given button defined in the specified VDJ.
///
/// nBtn can in the range 1-128
pub fn set_btn(value: i32, id: u32, n_btn: u8) {
    unsafe {
        ffi::SetBtn(value, id, n_btn);
    }
}

/// Write Value to a given discrete POV defined in the specified VDJ
///
/// Value can be one of the following:
///
/// 0: North (or Forwards)
///
/// 1: East (or Right)
///
/// 2: South (or backwards)
///
/// 3: West (or left)
///
/// -1: Neutral (Nothing pressed)
///
/// nPov selects the destination POV Switch. It can be 1 to 4
pub fn set_disc_pov(value: i32, id: u32, n_pov: u8) {
    unsafe {
        ffi::SetDiscPov(value, id, n_pov);
    }
}

/// Write Value to a given continuous POV defined in the specified VDJ
///
/// Value can be in the range: -1 to 35999.
///
/// It is measured in units of one-hundredth a degree.
///
/// -1 means Neutral (Nothing pressed).
///
/// nPov selects the destination POV Switch.
///
/// It can be 1 to 4
pub fn set_cont_pov(value: u32, id: u32, n_pov: u8) {
    unsafe {
        ffi::SetContPov(value, id, n_pov);
    }
}

pub enum VjoyError {
    InitializationError,
    OpenVjoyDeviceError,
}

pub struct VjoyApi;

impl VjoyApi {
    pub fn new() -> Result<Self, VjoyError> {
        if vjoy_enabled() && driver_match().0 {
            Ok(Self {})
        } else {
            Err(VjoyError::InitializationError)
        }
    }

    pub fn acquire_device(id: u32) -> Result<VjoyDevice, VjoyError> {
        if id > 16 {
            return Err(VjoyError::OpenVjoyDeviceError);
        }

        Ok(VjoyDevice::new(id)?)
    }
}

pub struct VjoyDevice {
    pub id: u32,
}

impl VjoyDevice {
    fn new(id: u32) -> Result<Self, VjoyError> {
        match get_vjd_status(id) {
            VJDStat::VjdStatOwned => return Ok(Self { id }),
            VJDStat::VjdStatFree => {
                if acquire_vjd(id) {
                    return Ok(Self { id });
                }
            }
            _ => {}
        }

        Err(VjoyError::OpenVjoyDeviceError)
    }
}

impl Drop for VjoyDevice {
    fn drop(&mut self) {
        relinquish_vjd(self.id);
    }
}
