
extern crate myo;

use myo::Event;

pub struct EventLogger;
impl myo::EventListener for EventLogger
{
    fn on_paired(&mut self, event: myo::PairedEvent) -> myo::HandlerResult
    {
        println!("{} Paired! {} {:?}", event.timestamp(), event.mac_address(), event.device_name());
        myo::HandlerResult::Continue
    }
    fn on_unpaired(&mut self, event: myo::UnpairedEvent) -> myo::HandlerResult
    {
        println!("{} Unpaired! {} {:?}", event.timestamp(), event.mac_address(), event.device_name());
        myo::HandlerResult::Continue
    }
    fn on_connected(&mut self, event: myo::ConnectedEvent) -> myo::HandlerResult
    {
        println!("{} Connected! {} {:?}: FW Version {:?}", event.timestamp(), event.mac_address(), event.device_name(), event.firmware_versions());
        myo::HandlerResult::Continue
    }
    fn on_disconnected(&mut self, event: myo::DisconnectedEvent) -> myo::HandlerResult
    {
        println!("{} Disconnected! {} {:?}", event.timestamp(), event.mac_address(), event.device_name());
        myo::HandlerResult::Continue
    }
    fn on_arm_synced(&mut self, event: myo::ArmSyncedEvent) -> myo::HandlerResult
    {
        let device = event.device();
        println!("{} {} Arm Synced! on {:?}, +x {:?}, warm? {:?}, estimated device rotation {} rad",
            event.timestamp(), device.raw_id(), event.arm(), event.xdirection(), event.warmup_state(), event.rotation_on_arm());
        device.request_rssi().unwrap();
        device.request_battery_level().unwrap();
        device.set_stream_emg(true).unwrap();
        myo::HandlerResult::Continue
    }
    fn on_arm_unsynced(&mut self, event: myo::ArmUnsyncedEvent) -> myo::HandlerResult
    {
        let device = event.device();
        println!("{} {} Arm Unsynced!", event.timestamp(), device.raw_id());
        device.set_stream_emg(false).unwrap();
        myo::HandlerResult::Continue
    }
    /*fn on_orientation_data(&mut self, event: myo::OrientationEvent) -> myo::HandlerResult
    {
        println!("{} {} Orientation Update: qrot {:?} acc {:?} gyro {:?}", event.timestamp(), event.device().raw_id(),
            event.q_orientation(), event.v_accelerometer(), event.v_gyroscope());
        myo::HandlerResult::Continue
    }*/
    fn on_pose(&mut self, event: myo::PoseEvent) -> myo::HandlerResult
    {
        println!("{} {} Pose Update: {:?}", event.timestamp(), event.device().raw_id(), event.pose());
        myo::HandlerResult::Continue
    }
    fn on_rssi_value(&mut self, event: myo::RSSIEvent) -> myo::HandlerResult
    {
        println!("{} {} RSSI: {}", event.timestamp(), event.device().raw_id(), event.rssi());
        myo::HandlerResult::Continue
    }
    fn on_unlocked(&mut self, event: myo::UnlockedEvent) -> myo::HandlerResult
    {
        println!("{} {} Unlocked!", event.timestamp(), event.device().raw_id());
        myo::HandlerResult::Continue
    }
    fn on_locked(&mut self, event: myo::LockedEvent) -> myo::HandlerResult
    {
        println!("{} {} Locked!", event.timestamp(), event.device().raw_id());
        myo::HandlerResult::Continue
    }
    fn on_emg_data(&mut self, event: myo::EMGEvent) -> myo::HandlerResult
    {
        println!("{} {} EMG: {:?}", event.timestamp(), event.device().raw_id(), event.emgs());
        myo::HandlerResult::Continue
    }
    fn on_battery_level(&mut self, event: myo::BatteryLevelEvent) -> myo::HandlerResult
    {
        println!("{} {} Battery Level: {}", event.timestamp(), event.device().raw_id(), event.battery_level());
        myo::HandlerResult::Continue
    }
    fn on_warmup_completed(&mut self, event: myo::WarmupCompletedEvent) -> myo::HandlerResult
    {
        println!("{} {} Warmup Completed! {:?}", event.timestamp(), event.device().raw_id(), event.result());
        myo::HandlerResult::Continue
    }
}

fn main()
{
    let hub = myo::Hub::init("rs.cterm2.myo.eventlog").unwrap();
    loop { hub.run(1000 / 20, &mut EventLogger).unwrap() }
}
