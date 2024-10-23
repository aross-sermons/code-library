// src/main.rs

use winit::{application::ApplicationHandler, window::Window};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::WindowEvent;
use std::ffi::CString;

use ash::{vk, Entry, Instance};

struct VulkanApp {
    entry: ash::Entry,
    instance: ash::Instance,
    required_device_queue_flags: [vk::QueueFlags],
    window: Option<Window>,
}

impl VulkanApp {
    pub fn new() -> Self {
        let (entry, instance) = VulkanApp::init_vulkan();

        let required_device_queue_flags = 
        Self {
            entry,
            instance,
            required_device_queue_flags,
            window: None,
        }
    }

    /// Initialize the Vulkan library and return an Entry and Instance
    pub fn init_vulkan() -> (ash::Entry, ash::Instance) {
        // Load the Vulkan library
        let entry = unsafe { Entry::load().unwrap() };

        // Application and engine info
        let app_name = CString::new("Vulkan App").unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();

        // Default application info
        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        // Instance create info
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        // Create Vulkan instance
        let instance = unsafe {
            entry.create_instance(&create_info, None)
            .expect("Failed to create Vulkan instance")
        };

        // Return entry and instance
        (entry, instance)
    }

    /// Function to pick the best physical device
    pub fn pick_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance.enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")
        };
    }

    /// Function to match chosen queue flags (required_queue_flags) to those avaiable to the device
    fn is_device_suitable(instance: &ash::Instance,
        device: vk::PhysicalDevice,
        required_queue_flags: &[vk::QueueFlags]
    ) -> bool {
        // Get the queue families of the physical device
        let queue_families = unsafe {
            instance.get_physical_device_queue_family_properties(device)
        };
        // For each flag in required_queue_flags...
        required_queue_flags.iter().all(|&required_flags| {
            // Iterate through present queue_families to check if the flag is satisfied
            queue_families.iter().any(|q| q.queue_flags.contains(required_flags))
        })
    }
    
    /// Destroy Vulkan instance
    pub fn cleanup(&self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

impl ApplicationHandler for VulkanApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }
    
    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}

fn main() {
    // Create an EventLoop to handle window creation and interactions
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    // Create an App, which implements winit::application::ApplicationHandler for event handling
    let mut vapp = VulkanApp::new();
    // Run the event loop using the created App
    event_loop.run_app(&mut vapp);

    // Destroy Vulkan instance when done
    vapp.cleanup();
}
