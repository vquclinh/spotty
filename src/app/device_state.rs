use crate::app::types::*;
use crate::network::models::*;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct DeviceState {
    pub online_devices: StatefulList<Device>,
    pub local_device_idx: Option<usize>,
    pub active_device_idx: Option<usize>,
    
    pub last_area: Rect,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            online_devices: StatefulList::new(),
            local_device_idx: None,
            active_device_idx: None,
            
            last_area: Rect::default()
        }
    }
}

impl DeviceState {
    pub fn active_device_id(&self) -> Option<String> {
        self.active_device_idx
            .and_then(|idx| self.online_devices.items.get(idx))
            .and_then(|d| d.id.clone())
    }

    pub fn local_device_id(&self) -> Option<String> {
        self.local_device_idx
            .and_then(|idx| self.online_devices.items.get(idx))
            .and_then(|d| d.id.clone())
    }

    pub fn is_active_device(&self) -> bool {
        self.active_device_id() == self.local_device_id()
    }
}