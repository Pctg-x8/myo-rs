//! libmyo C API Declarations.
//! Copyright (C) 2013-2014 Thalmic Labs Inc.
//! Distributed under the Myo SDK license agreement. See LICENSE.txt for details.

#![allow(non_camel_case_types)]

use libc::*;

pub type libmyo_hub_t = *mut c_void;

// Error Handling
/// Function result codes.
/// All libmyo functions that can fail return a value of this type.
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ResultCode
{
    Success, Error, ErrorInvalidArgument, Runtime
}

/// Opaque handle to detailed error information.
pub type libmyo_error_details_t = *mut c_void;

// Strings
/// Opaque string.
pub type libmyo_string_t = *mut c_void;

/// Locking policies
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum LockingPolicy
{
    /// Pose events are always send
    None,
    /// Post events are not sent while a Myo is locked.
    Standard
}

// Myo instances
/// Opaque type corresponding to a known Myo device.
pub type libmyo_myo_t = *mut c_void;

/// Types of vibration
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum VibrationType
{
    Short, Medium, Long
}

/// EMG streaming modes.
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum EMGStreamingMode
{
    /// Do not send EMG data.
    Disabled,
    /// Send EMG data.
    Enabled
}

// Post recognition.
/// Supported poses.
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Pose
{
    /// Rest pose.
    rest = 0,
    /// User is making a fist.
    fist = 1,
    /// User has an open palm rotated towards the posterior of their wrist.
    wave_in = 2,
    /// User has an open palm rotated towards the anterior of their wrist.
    wave_out = 3,
    /// User has an open palm with their fingers spread away from each other.
    fingers_spread = 4,
    /// User tapped their thumb and middle finger together twice in succession.
    double_tap = 5,
    /// Number of poses supported; not a valid pose.
    num_poses,
    /// Unknown pose.
    unknown = 0xffff
}

// Myo locking mechanism
/// Valid unlock types.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum UnlockType
{
    /// Unlock for a fixed period of time.
    Timed = 0,
    /// Unlock until explicitly told to re-lock.
    Hold = 1
}

/// User action types
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum UserActionType
{
    /// User did a single, discrete action, such as pausing a video.
    Single = 0
}

// Event Handling
/// Types of events.
#[repr(u32)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum EventType
{
    /// Successfully paired with a Myo.
    Paired,
    /// Successfully unpaired from a Myo.
    Unpaired,
    /// A Myo has successfully connected.
    Connected,
    /// A Myo has been disconnected.
    Disconnected,
    /// A Myo has recognized that the sync gesture has been successfully performed.
    ArmSynced,
    /// A Myo has been moved or removed from the arm.
    ArmUnsynced,
    /// Orientation data has been recevied.
    Orientation,
    /// A change in pose has been detected. @see libmyo_pose_t.
    Pose,
    /// An RSSI value has been received.
    RSSI,
    /// A Myo has become unlocked.
    Unlocked,
    /// A Myo has become locked.
    Locked,
    /// EMG data has been received.
    EMG,
    /// A battery level value has been received.
    BatteryLevel,
    /// The warmup period has completed.
    WarmupCompleted
}

/// Information about an event.
pub type libmyo_event_t = *const c_void;

/// Components of version.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum VersionComponent
{
    /// Major version.
    Major,
    /// Minor version.
    Minor,
    /// Patch version.
    Patch,
    /// Hardware revision.
    HardwareRevision
}

/// Hardware revisions.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum HardwareRevision
{
    /// Alpha units
    RevC = 1,
    /// Consumer units
    RevD = 2
}

/// Enumeration identifying a right arm or left arm. @see libmyo_event_get_arm()
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum Arm
{
    /// Myo is on the right arm.
    Right,
    /// Myo is on the left arm.
    Left,
    /// Unknown arm.
    Unknown
}
/// Possible directions for Myo's +x axis relative to a user's arm.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum XDirection
{
    /// Myo's +x axis is pointing toward the user's wrist.
    TowardWrist,
    /// Myo's +x axis is pointing toward the user's elbow.
    TowardElbow,
    /// Unknown +x axis direction.
    Unknown
}
/// Possible warmup states for Myo.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum WarmupState
{
    /// Unknown warm up state.
    Unknown = 0,
    /// Myo needs to warm up.
    Cold = 1,
    /// Myo is already in a warmed up state.
    Warm = 2
}
/// Possible warmup results for Myo.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum WarmupResult
{
    /// Unknown warm up result.
    Unknown = 0,
    /// The warm up period has completed successfully.
    Success = 1,
    /// The warm up period timed out.
    Timeout = 2
}
/// Index into orientation data, which is provided as a quaternion.
/// Orientation data is returned as a unit quaternion of floats, represented as `w + x * i + y * j + z * k`
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum OrientationIndex
{
    /// First component of the quaternion's vector part
    X = 0,
    /// Second component of the quaternion's vector part
    Y = 1,
    /// Third component of the quaternion's vector part
    Z = 2,
    /// Scalar component of the quaternion
    W = 3
}
/// Retrun type for event handlers.
#[repr(C)] #[derive(Debug, PartialEq, Eq, Clone, Copy)] pub enum HandlerResult
{
    /// Continue processing events
    Continue,
    /// Stop processing events
    Stop
}
/// Callback function type to handle events as they occur from libmyo_run()
pub type libmyo_handler_t = extern "system" fn(user_data: *mut c_void, event: libmyo_event_t) -> HandlerResult;

#[cfg_attr(target_pointer_width = "32", link(name = "myo32"))]
#[cfg_attr(target_pointer_width = "64", link(name = "myo64"))]
extern "system"
{
    /// Return a null-terminated string with a detailed error message.
    pub fn libmyo_error_cstring(details: libmyo_error_details_t) -> *const c_char;
    /// Returns the kind of error that occured.
    pub fn libmyo_error_kind(details: libmyo_error_details_t) -> ResultCode;
    /// Free the resources allocated by an error object.
    pub fn libmyo_free_error_details(details: libmyo_error_details_t);

    /// Return a null-terminated string from the opaque string.
    pub fn libmyo_string_c_str(s: libmyo_string_t) -> *const c_char;
    /// Free the resources allocated by the string object.
    pub fn libmyo_string_free(s: libmyo_string_t);

    // MAC address utilities
    /// Retrieve the string representation of a MAC address in hex.
    /// Returns a string in the format of 00-00-00-00-00-00.
    pub fn libmyo_mac_address_to_string(addr: u64) -> libmyo_string_t;
    /// Retrieve the MAC address from a null-terminated string in the format of 00-00-00-00-00-00.
    /// Returns 0 if the string does not match the format.
    pub fn libmyo_string_to_mac_address(s: *const c_char) -> u64;

    // Hub instance
    /// Initoialize a connection to the hub.
    /// \a application_identifier must follow a reverse domain name format (ex. com.domainname.appname). Application
    /// identifiers can be formed from the set of alphanumeric ASCII characters (a-z, A-Z, 0-9). The hyphen (-) and
    /// underscore (_) characters are permitted if they are not adjacent to a period (.) character (i.e. not at the start or
    /// end of each segment), but are not permitted in the top-level domain. Application identifiers must have three or more
    /// segments. For example, if a company's domain is example.com and the application is named hello-world, one could use
    /// "com.example.hello-world" as a valid application identifier. \a application_identifier can be NULL or empty.
    /// ## returns
    /// libmyo_success if the connection is successfully established, otherwise:
    ///  - libmyo_error_runtime if a connection could not be established
    ///  - libmyo_error_invalid_argument if \a out_hub is NULL
    ///  - libmyo_error_invalid_argument if \a application_identifier is longer than 255 characters
    ///  - libmyo_errr_invalid_argument if \a application_identifier is not in the proper reverse domain name format
    pub fn libmyo_init_hub(out_hub: *mut libmyo_hub_t, application_identifier: *const c_char, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Free the resources allocated to a hub.
    /// ## returns
    /// libmyo_success if shutdown is successful, otherwise:
    ///  - libmyo_error_invalid_argument if \a hub is NULL
    ///  - libmyo_error if \a hub is not a valid hub
    pub fn libmyo_shutdown_hub(hub: libmyo_hub_t, out_error: *mut libmyo_error_details_t) -> ResultCode;

    // Locking policies
    /// Set the locking policy for Myos connected to the hub.
    /// ## returns
    /// libmyo_success if the locking policy is successfully set, otherwise
    ///  - libmyo_error_invalid_argument if `hub` is NULL
    ///  - libmyo_error if `hub` is not a valid hub
    pub fn libmyo_set_locking_policy(hub: libmyo_hub_t, locking_policy: LockingPolicy, out_error: *mut libmyo_error_details_t) -> ResultCode;

    // Myo instances
    // Retrieve the MAC address of a Myo.
    // The MAC address is unique to the physical Myo, and is a 48-bit number.
    // [Deprecated?]pub fn libmyo_get_mac_address(myo: libmyo_myo_t) -> u64;
    /// Vibrate the given myo.
    /// Can be called when a Myo is paired.
    /// ## returns
    /// libmyo_success if the Myo successfully vibrated, otherwise
    ///  - libmyo_error_invalid_argument if `myo` is NULL
    pub fn libmyo_vibrate(myo: libmyo_myo_t, vtype: VibrationType, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Request the RSSI for a given myo.
    /// Can be called when a Myo is paired. A libmyo_event_rssi event will likely be generated with the value of the RSSI.
    /// ## returns
    /// libmyo_success if the Myo successfully got the RSSI, otherwise
    ///  - libmyo_error_invalid_argument if `myo` is NULL
    pub fn libmyo_request_rssi(myo: libmyo_myo_t, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Request the battery level for a given Myo.
    /// A libmyo_event_battery_level event will be generated with the value of the battery level.
    /// @returns libmyo_success if the Myo successfully requested the battery level, otherwise
    ///  - libmyo_error_invalid_argument if \a myo is NULL
    pub fn libmyo_request_battery_level(myo_opq: libmyo_myo_t, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Set whether or not to stream EMG data for a given myo.
    /// Can be called when a Myo is paired.
    /// @returns libmyo_success if the EMG mode was set successfully, otherwise
    ///  - libmyo_error_invalid_argument if \a myo is NULL
    pub fn libmyo_set_stream_emg(myo: libmyo_myo_t, emg: EMGStreamingMode, out_error: *mut libmyo_error_details_t) -> ResultCode;

    // Myo locking mechanism
    /// Unlock the given Myo.
    /// Can be called when a Myo is paired. A libmyo_event_unlocked event will be generated if the Myo was locked.
    /// ## returns
    /// libmyo_success if the Myo was successfully unlocked, otherwise
    ///  - libmyo_error_invalid_argument if `myo` is NULL.
    pub fn libmyo_myo_unlock(myo: libmyo_myo_t, utype: UnlockType, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Lock the given Myo immediately.
    /// Can be called when a Myo is paired. A libmyo_event_locked event will be generated if the Myo was unlocked.
    /// ## returns
    /// libmyo_success if the Myo was successfully locked, otherwise
    ///  - libmyo_error_invalid_argument if `myo` is NULL.
    pub fn libmyo_myo_lock(myo: libmyo_myo_t, out_error: *mut libmyo_error_details_t) -> ResultCode;
    /// Notify the given Myo that a user action was recognized.
    /// Can be called when a Myo is paired. Will cause Myo to vibrate.
    /// @returns libmyo_success if the Myo was successfully notified, otherwise
    ///  - libmyo_error_invalid_argument if \a myo is NULL
    pub fn libmyo_myo_notify_user_action(myo: libmyo_myo_t, atype: UserActionType, out_error: *mut libmyo_error_details_t) -> ResultCode;

    // Event Handling
    /// Retrieve the type of an event.
    pub fn libmyo_event_get_type(event: libmyo_event_t) -> u32;
    /// Retrieve the timestamp of an event.
    /// @see libmyo_now() for details on timestamps.
    pub fn libmyo_event_get_timestamp(event: libmyo_event_t) -> u64;
    /// Retrieve the Myo associated with an event.
    pub fn libmyo_event_get_myo(event: libmyo_event_t) -> libmyo_myo_t;
    /// Retrieve the MAC address of the myo associated with an event.
    pub fn libmyo_event_get_mac_address(event_opq: libmyo_event_t) -> u64;
    /// Retrieve the name of the myo associated with an event.
    /// Caller must free the returned string. @see libmyo_string functions.
    pub fn libmyo_event_get_myo_name(event: libmyo_event_t) -> libmyo_string_t;
    /// Retrieve the Myo armband's firmware version from the event.
    /// Valid for libmyo_event_paired and libmyo_event_connected events.
    pub fn libmyo_event_get_firmware_version(event: libmyo_event_t, c: VersionComponent) -> c_uint;
    /// Retrieve the arm associated with an event.
    /// Valid for libmyo_event_arm_synced events only.
    pub fn libmyo_event_get_arm(event: libmyo_event_t) -> Arm;
    /// Retrieve the x-direction associated with an event.
    /// The x-direction specifies which way Myo's +x axis is pointing relative to the user's arm.
    /// Valid for libmyo_event_arm_synced events only.
    pub fn libmyo_event_get_x_direction(event: libmyo_event_t) -> XDirection;
    /// Retrieve the warmup state of the Myo associated with an event.
    /// Valid for libmyo_event_arm_synced events only.
    pub fn libmyo_event_get_warmup_state(event: libmyo_event_t) -> WarmupState;
    /// Retrieve the warmup result of the Myo associated with an event.
    /// Valid for libmyo_event_warmup_completed events only.
    pub fn libmyo_event_get_warmup_result(event: libmyo_event_t) -> WarmupResult;
    /// Retrieve the estimated rotation of Myo on the user's arm after a sync.
    /// The values specifies the rotation of the myo on the arm (0 - logo facing down, pi - logo facing up)
    /// Only supported by FW 1.3.x and above (older firmware will always report 0 for the rotation)
    /// Valid for libmyo_event_arm_synced events only.
    pub fn libmyo_event_get_rotation_on_arm(event: libmyo_event_t) -> c_float;
    /// Retrieve orientation data associated with an event.
    /// Valid for libmyo_event_orientation events only.
    /// @see libmyo_orientation_index
    pub fn libmyo_event_get_orientation(event: libmyo_event_t, index: OrientationIndex) -> c_float;
    /// Retrieve raw accelerometer data associated with an event in units of g.
    /// Valid for libmyo_event_orientation events only.
    /// Requires `index < 3`
    pub fn libmyo_event_get_accelerometer(event: libmyo_event_t, index: c_uint) -> c_float;
    /// Retrieve raw gyroscope data associated with an event in units of deg/s.
    /// Valid for libmyo_event_orientation events only.
    /// Requires `index < 3`
    pub fn libmyo_event_get_gyroscope(event: libmyo_event_t, index: c_uint) -> c_float;
    /// Retrieve the pose associated with an event.
    /// Valid for libmyo_event_pose events only.
    pub fn libmyo_event_get_pose(event: libmyo_event_t) -> Pose;
    /// Retrieve the RSSI associated with an event.
    /// Valid for libmyo_event_rssi events only.
    pub fn libmyo_event_get_rssi(event: libmyo_event_t) -> i8;
    /// Retrieve the battery level of the Myo armband associated with an event.
    /// Only valid for libmyo_event_battery_level event.
    pub fn libmyo_event_get_battery_level(event: libmyo_event_t) -> u8;
    /// Retrieve an EMG data point associated with an event.
    /// Valid for libmyo_event_emg events only.
    /// @a sensor must be smaller than 8.
    pub fn libmyo_event_get_emg(event: libmyo_event_t, sensor: c_uint) -> i8;
    /// Process events and call the provided callback as they occur.
    /// Runs for up to approximately `duration_ms` milliseconds or until a called handler returns libmyo_handler_stop.
    /// ## returns
    /// libmyo_success after a successful run, otherwise
    ///  - libmyo_error_invalid_argument if `hub` is NULL
    ///  - libmyo_error_invalid_argument if `handler` is NULL
    pub fn libmyo_run(hub: libmyo_hub_t, duration_ms: c_uint, handler: libmyo_handler_t, user_data: *mut c_void, out_error: *mut libmyo_error_details_t) -> ResultCode;
}
