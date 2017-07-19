//! Myo Armband

#![feature(raw)]

extern crate libc;

mod ffi;
pub use ffi::{
    ResultCode, LockingPolicy, VibrationType, UnlockType, VersionComponent, EventType,
    Arm, XDirection, WarmupState, WarmupResult, Pose, OrientationIndex, HandlerResult,
    HardwareRevision
};
use std::ffi::CStr;

/// Operation Result
pub type Result<T> = std::result::Result<T, ErrorDetails>;

/// Owned Myo String
pub struct MyoString(ffi::libmyo_string_t);
impl MyoString
{
    pub fn c_str(&self) -> &CStr
    {
        unsafe { CStr::from_ptr(ffi::libmyo_string_c_str(self.0)) }
    }
}
impl Drop for MyoString
{
    fn drop(&mut self)
    {
        unsafe { ffi::libmyo_string_free(self.0) };
    }
}
impl std::fmt::Debug for MyoString
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        self.c_str().fmt(fmt)
    }
}

/// Owned Error Details
pub struct ErrorDetails(ffi::libmyo_error_details_t);
impl ErrorDetails
{
    pub fn message(&self) -> &CStr
    {
        unsafe { CStr::from_ptr(ffi::libmyo_error_cstring(self.0)) }
    }
    pub fn kind(&self) -> ResultCode
    {
        unsafe { ffi::libmyo_error_kind(self.0) }
    }
}
impl Drop for ErrorDetails
{
    fn drop(&mut self)
    {
        unsafe { ffi::libmyo_free_error_details(self.0) };
    }
}
impl std::fmt::Debug for ErrorDetails
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(fmt, "libmyo Error Details({:?}: {:?})", self.kind(), self.message())
    }
}
impl std::fmt::Display for ErrorDetails
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        std::fmt::Debug::fmt(self, fmt)
    }
}
impl std::error::Error for ErrorDetails
{
    fn description(&self) -> &str { "libmyo Error" }
}

/// MAC Address
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MACAddress(u64);
impl std::fmt::Display for MACAddress
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(fmt, "{}", MyoString(unsafe { ffi::libmyo_mac_address_to_string(self.0) }).c_str().to_str().unwrap())
    }
}

/// Wrapper Macros
macro_rules! DefWrapperFunc
{
    (pub fn $ename: ident ( $($narg: ident : $targ: ty),* ) = $fname: ident ( $($farg: expr),* )) =>
    {
        pub fn $ename(&self $(, $narg: $targ)*) -> Result<()>
        {
            let mut e = std::ptr::null_mut();
            let r = unsafe { ffi::$fname(self.0 $(, $farg)*, &mut e) };
            if r == ResultCode::Success { Ok(()) } else { Err(ErrorDetails(e)) }
        }
    };
    (pub fn $ename: ident ( $($narg: ident : $targ: ty),* ) -> $rtype: ty = $fname: ident ( $($farg: expr),* )) =>
    {
        pub fn $ename(&self $(, $narg: $targ)*) -> $rtype
        {
            unsafe { ffi::$fname(self.0 $(, $farg)*) as _ }
        }
    }
}

/// Event Listener
pub trait EventListener
{
    /// Called when successfully paired with a Myo.
    fn on_paired(&mut self, event: PairedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when successfully unpaired from a Myo.
    fn on_unpaired(&mut self, event: UnpairedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has successfully connected.
    fn on_connected(&mut self, event: ConnectedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has been disconnected.
    fn on_disconnected(&mut self, event: DisconnectedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has recognized that the sync gesture has been successfully performed.
    fn on_arm_synced(&mut self, event: ArmSyncedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has been moved or removed from the arm.
    fn on_arm_unsynced(&mut self, event: ArmUnsyncedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when orientation data has been received.
    fn on_orientation_data(&mut self, event: OrientationEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a change in pose has been detected.
    fn on_pose(&mut self, event: PoseEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when an RSSI value has been received.
    fn on_rssi_value(&mut self, event: RSSIEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has become unlocked.
    fn on_unlocked(&mut self, event: UnlockedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a Myo has become locked.
    fn on_locked(&mut self, event: LockedEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when EMG data has been received.
    fn on_emg_data(&mut self, event: EMGEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when a battery level value has been received.
    fn on_battery_level(&mut self, event: BatteryLevelEvent) -> HandlerResult { HandlerResult::Continue }
    /// Called when the warmup period has completed.
    fn on_warmup_completed(&mut self, event: WarmupCompletedEvent) -> HandlerResult { HandlerResult::Continue }
}

/// Store for Trait Object(stable passing for std::raw::TraitObject)
struct TraitObjectStore<'a, T: 'a + ?Sized>(&'a mut T);

/// Hub
pub struct Hub(ffi::libmyo_hub_t);
impl Hub
{
    /// Initialize
    pub fn init<AppID: Into<Vec<u8>>>(application_identifier: AppID) -> Result<Self>
    {
        let (mut h, mut e) = (std::ptr::null_mut(), std::ptr::null_mut());
        let appid = std::ffi::CString::new(application_identifier).unwrap();
        let r = unsafe { ffi::libmyo_init_hub(&mut h, appid.as_ptr(), &mut e) };
        if r == ResultCode::Success { Ok(Hub(h)) } else { Err(ErrorDetails(e)) }
    }

    DefWrapperFunc!(pub fn set_locking_policy(locking_policy: LockingPolicy) = libmyo_set_locking_policy(locking_policy));
    /// Process Events and call the provided callback as they occur
    pub fn run(&self, duration_ms: u32, listener: &mut EventListener) -> Result<()>
    {
        let mut to = TraitObjectStore(listener);
        let mut e = std::ptr::null_mut();
        let r = unsafe { ffi::libmyo_run(self.0, duration_ms as _, Self::run_internal_handler, &mut to as *mut TraitObjectStore<_> as *mut libc::c_void, &mut e) };
        if r == ResultCode::Success { Ok(()) } else { Err(ErrorDetails(e)) }
    }

    extern "system" fn run_internal_handler(elptr: *mut libc::c_void, event: ffi::libmyo_event_t) -> HandlerResult
    {
        let el: &mut EventListener = unsafe { std::mem::transmute::<_, &mut TraitObjectStore<'static, EventListener>>(&mut *elptr).0 };
        match unsafe { std::mem::transmute(ffi::libmyo_event_get_type(event)) }
        {
            EventType::Paired => el.on_paired(PairedEvent(event)),
            EventType::Unpaired => el.on_unpaired(UnpairedEvent(event)),
            EventType::Connected => el.on_connected(ConnectedEvent(event)),
            EventType::Disconnected => el.on_disconnected(DisconnectedEvent(event)),
            EventType::ArmSynced => el.on_arm_synced(ArmSyncedEvent(event)),
            EventType::ArmUnsynced => el.on_arm_unsynced(ArmUnsyncedEvent(event)),
            EventType::Orientation => el.on_orientation_data(OrientationEvent(event)),
            EventType::Pose => el.on_pose(PoseEvent(event)),
            EventType::RSSI => el.on_rssi_value(RSSIEvent(event)),
            EventType::Unlocked => el.on_unlocked(UnlockedEvent(event)),
            EventType::Locked => el.on_locked(LockedEvent(event)),
            EventType::EMG => el.on_emg_data(EMGEvent(event)),
            EventType::BatteryLevel => el.on_battery_level(BatteryLevelEvent(event)),
            EventType::WarmupCompleted => el.on_warmup_completed(WarmupCompletedEvent(event))
        }
    }
}
impl Drop for Hub
{
    /// Shutdown
    fn drop(&mut self)
    {
        let mut e = std::ptr::null_mut();
        let r = unsafe { ffi::libmyo_shutdown_hub(self.0, &mut e) };
        if r != ResultCode::Success
        {
            panic!("Error on drop: {:?}", ErrorDetails(e).message());
        }
    }
}

/// Myo Armband
#[derive(Debug)]
pub struct Armband(ffi::libmyo_myo_t);
impl Armband
{
    /// Raw ID
    pub fn raw_id(&self) -> usize { unsafe { std::mem::transmute(self.0) } }
    /// Vibrate
    DefWrapperFunc!(pub fn vibrate(vtype: VibrationType) = libmyo_vibrate(vtype));
    /// Asynchronous Request: RSSI
    DefWrapperFunc!(pub fn request_rssi() = libmyo_request_rssi());
    /// Asynchronous Request: Battery Level
    DefWrapperFunc!(pub fn request_battery_level() = libmyo_request_battery_level());
    /// Stream EMG Data
    DefWrapperFunc!(pub fn set_stream_emg(stream_emg_data: bool) =
        libmyo_set_stream_emg(if stream_emg_data { ffi::EMGStreamingMode::Enabled } else { ffi::EMGStreamingMode::Disabled }));

    // Locking Mechanism
    /// Lock Armband
    DefWrapperFunc!(pub fn lock() = libmyo_myo_lock());
    /// Unlock Armband
    DefWrapperFunc!(pub fn unlock(unlock_type: UnlockType) = libmyo_myo_unlock(unlock_type));
    /// Notify Myo device that a user action was recognized.
    /// Device will vibrate.
    DefWrapperFunc!(pub fn notify_user_action() = libmyo_myo_notify_user_action(ffi::UserActionType::Single));
}

/// Event Object
pub trait Event
{
    /// Retrieve Event Handle
    fn handle(&self) -> ffi::libmyo_event_t;
    
    /// Event Type
    fn event_type(&self) -> EventType { unsafe { std::mem::transmute(ffi::libmyo_event_get_type(self.handle())) } }
    /// Timestamp
    fn timestamp(&self) -> u64 { unsafe { ffi::libmyo_event_get_timestamp(self.handle()) } }
    /// Myo Device
    fn device(&self) -> Armband { Armband(unsafe { ffi::libmyo_event_get_myo(self.handle()) }) }
    /// MAC Address
    fn mac_address(&self) -> MACAddress { MACAddress(unsafe { ffi::libmyo_event_get_mac_address(self.handle()) }) }
    /// Myo Name
    fn device_name(&self) -> MyoString { MyoString(unsafe { ffi::libmyo_event_get_myo_name(self.handle()) }) }
}
macro_rules! DefEvent
{
    ($t: ident) =>
    {
        pub struct $t(ffi::libmyo_event_t);
        impl Event for $t { fn handle(&self) -> ffi::libmyo_event_t { self.0 } }
    }
}
macro_rules! DefEventParamWrapper
{
    (pub property<$obj: ty> $pn: ident [$($an: ident: $at: ty),*]: $pt: ty = $f: ident ($($fan: expr),*)) =>
    {
        impl $obj
        {
            pub fn $pn(&self $(, $an: $at)*) -> $pt
            {
                unsafe { ffi::$f(self.0 $(, $fan)*) as _ }
            }
        }
    };
    (pub property<$obj: ty> $pn: ident: $pt: ty = $f: ident ($($fan: expr),*)) =>
    {
        impl $obj
        {
            pub fn $pn(&self) -> $pt
            {
                unsafe { ffi::$f(self.0 $(, $fan)*) as _ }
            }
        }
    };
}

/// Successfully paired with a Myo.
DefEvent!(PairedEvent);
/// Successfully unpaired from a Myo.
DefEvent!(UnpairedEvent);
/// A Myo has successfully connected.
DefEvent!(ConnectedEvent);
/// A Myo has been disconnected.
DefEvent!(DisconnectedEvent);
/// A Myo has recognized that the sync gesture has been successfully performed.
DefEvent!(ArmSyncedEvent);
/// A Myo has been moved or removed from the arm.
DefEvent!(ArmUnsyncedEvent);
/// Orientation data has been received.
DefEvent!(OrientationEvent);
/// A change in pose has been detected.
DefEvent!(PoseEvent);
/// An RSSI value has been received.
DefEvent!(RSSIEvent);
/// A Myo has become unlocked.
DefEvent!(UnlockedEvent);
/// A Myo has become locked.
DefEvent!(LockedEvent);
/// EMG data has been received.
DefEvent!(EMGEvent);
/// A battery level value has been received.
DefEvent!(BatteryLevelEvent);
/// The warmup period has completed.
DefEvent!(WarmupCompletedEvent);

impl PairedEvent
{
    /// Firmware Version
    DefWrapperFunc!(pub fn firmware_version(component: VersionComponent) -> u32 = libmyo_event_get_firmware_version(component));

    /// Firmware Version Set
    pub fn firmware_versions(&self) -> (u32, u32, u32, HardwareRevision)
    {
        (self.firmware_version(VersionComponent::Major),
        self.firmware_version(VersionComponent::Minor),
        self.firmware_version(VersionComponent::Patch),
        unsafe { std::mem::transmute(self.firmware_version(VersionComponent::HardwareRevision)) })
    }
}
impl ConnectedEvent
{
    /// Firmware Version
    DefWrapperFunc!(pub fn firmware_version(component: VersionComponent) -> u32 = libmyo_event_get_firmware_version(component));

    /// Firmware Version Set
    pub fn firmware_versions(&self) -> (u32, u32, u32, HardwareRevision)
    {
        (self.firmware_version(VersionComponent::Major),
        self.firmware_version(VersionComponent::Minor),
        self.firmware_version(VersionComponent::Patch),
        unsafe { std::mem::transmute(self.firmware_version(VersionComponent::HardwareRevision)) })
    }
}

impl ArmSyncedEvent
{
    /// Arm Side
    DefWrapperFunc!(pub fn arm() -> Arm = libmyo_event_get_arm());
    /// +x Direction
    DefWrapperFunc!(pub fn xdirection() -> XDirection = libmyo_event_get_x_direction());
    /// Warming up state
    DefWrapperFunc!(pub fn warmup_state() -> WarmupState = libmyo_event_get_warmup_state());
    /// Eastimated Rotation of Myo on the user's arm
    DefWrapperFunc!(pub fn rotation_on_arm() -> f32 = libmyo_event_get_rotation_on_arm());
}
DefEventParamWrapper!(pub property<WarmupCompletedEvent> result: WarmupResult = libmyo_event_get_warmup_result());

/// Accelerometer/Gyroscope Index Value
#[repr(u8)] pub enum CoordinateIndex
{
    X = 0, Y = 1, Z = 2
}

impl OrientationEvent
{
    /// Orientation data
    DefWrapperFunc!(pub fn orientation(index: OrientationIndex) -> f32 = libmyo_event_get_orientation(index));
    /// Accelerometer data
    DefWrapperFunc!(pub fn accelerometer(index: CoordinateIndex) -> f32 = libmyo_event_get_accelerometer(index as _));
    /// Gyroscope data
    DefWrapperFunc!(pub fn gyroscope(index: CoordinateIndex) -> f32 = libmyo_event_get_gyroscope(index as _));

    // Support Funcs
    /// Orientation Data
    pub fn q_orientation(&self) -> (f32, f32, f32, f32)
    {
        (self.orientation(OrientationIndex::X), self.orientation(OrientationIndex::Y),
            self.orientation(OrientationIndex::Z), self.orientation(OrientationIndex::W))
    }
    /// Accelerometer Data
    pub fn v_accelerometer(&self) -> (f32, f32, f32)
    {
        (self.accelerometer(CoordinateIndex::X), self.accelerometer(CoordinateIndex::Y), self.accelerometer(CoordinateIndex::Z))
    }
    /// Gyroscope Data
    pub fn v_gyroscope(&self) -> (f32, f32, f32)
    {
        (self.gyroscope(CoordinateIndex::X), self.gyroscope(CoordinateIndex::Y), self.gyroscope(CoordinateIndex::Z))
    }
}
DefEventParamWrapper!(pub property<PoseEvent> pose: Pose = libmyo_event_get_pose());
DefEventParamWrapper!(pub property<RSSIEvent> rssi: i8 = libmyo_event_get_rssi());
DefEventParamWrapper!(pub property<BatteryLevelEvent> battery_level: u8 = libmyo_event_get_battery_level());
impl EMGEvent
{
    /// EMG Data
    DefWrapperFunc!(pub fn emg(sensor: u8) -> i8 = libmyo_event_get_emg(sensor as _));

    /// EMGs
    pub fn emgs(&self) -> [i8; 8]
    {
        let mut a = [0; 8]; for n in 0 .. 8 { a[n] = self.emg(n as _); } a
    }
}

