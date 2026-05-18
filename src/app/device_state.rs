use crate::app::types::*;
use crate::network::models::*;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct DeviceState {
    pub online_devices: StatefulList<Device>,
    pub local_device_idx: Option<usize>,
    
    pub last_area: Rect,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            online_devices: StatefulList::new(),
            local_device_idx: None,
            
            last_area: Rect::default()
        }
    }
}

impl DeviceState {
    pub fn active_device_id(&self) -> Option<String> {
        for d in self.online_devices.items.iter() {
            if d.is_active {
                return d.id.clone();
            }
        }
        None
    }

    pub fn local_device_id(&self) -> Option<String> {
        self.local_device_idx
            .and_then(|idx| self.online_devices.items.get(idx))
            .and_then(|d| d.id.clone())
    }

    pub fn is_active_device(&self) -> bool {
        let active_id = self.active_device_id();
        active_id.is_some() && active_id == self.local_device_id()
    }

    // Returns the index of the active device in the list
    pub fn active_device_idx(&self) -> Option<usize> {
        let active_id = self.active_device_id();
        self.online_devices.items.iter().position(|i| i.id == active_id)
    }

    pub fn device_id_from_idx(&self, idx: usize) -> Option<String> {
        self.online_devices
            .items
            .get(idx)
            .and_then(|d| d.id.clone())
    }
}