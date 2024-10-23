// src/vulkan_renderer.rs

use ash::{vk, Entry};
use winit::window::Window;
use std::ffi::CString;


pub struct VulkanRenderer {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKRR,
}

impl VulkanRenderer {
    /// Initialize a new VulkanRenderer
    pub fn new() -> Self {
        // Entry::linked() automatically finds the best library for the host machine
        let entry = ash::Entry::linked();
        // Create Vulkan instance
        let instance = Self::create_instance(&entry);

        Self {
            entry,
            instance,
        }
    }

    /// Function to create Vulkan instance
    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        // Create app_info
        let app_info = vk::ApplicationInfo {
            p_application_name: CString::new("Vulkan App").unwrap().as_ptr(), // No functional impact, used to differentiate this renderer from others
            application_version: vk::make_api_version(0, 1, 0, 0), // You may want to use a newer version
            p_engine_name: CString::new("No Engine").unwrap().as_ptr(), // No functional impact, used to differentiate this renderer from others
            engine_version: vk::make_api_version(0, 1, 0, 0), // You may want to use a newer version
            api_version: vk::make_api_version(0, 1, 0, 0), // You may want to use a newer version
            ..Default::default()
        };
        // Create instance create_info
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        // Return the instance
        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance")
        }
    }

    /// Function to create a Vulkan surface
    fn create_surface(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &Window,
    ) -> vk::SurfaceKHR {
        // Using winit to create a surface based on the window platform
        unsafe { ash_window::create_surface(entry, instance, window, None).unwrap() }
    }

    /// Function to find a grapics device that matched queue family and surface constraints
    fn pick_physical_device(
        instance: &ash::Instance,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
        required_queue_flags: &[vk::QueueFlags],
    ) -> vk::PhysicalDevice {
        unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")
                .into_iter()
                .find(|&device| {
                    let queue_families = instance.get_physical_device_queue_family_properties(device);

                    queue_families.iter().enumerate().any(|(i, queue_family)| {
                        required_queue_flags.iter().any(|&flag| queue_family.queue_flags.contains(flag))
                            && surface_loader
                                .get_physical_device_surface_support(device, i as u32, surface)
                                .unwrap()
                    })
                })
                .expect("Failed to find a suitable graphics device")
        }
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
    ) -> (ash::Device, vk::Queue) {
        let queue_priority = 1.0;
        let queue_info = vk::DeviceQueueCreateInfo {
            queue_family_index: 0,
            queue_count: 1,
            p_queue_priorities: &queue_priority,
            ..Default::default()
        };
        let device_create_info = vk::DeviceCreateInfo {
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_info,
            ..Default::default()
        };
        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };
        let graphics_queue = unsafe { device.get_device_queue(0, 0) };

        (device, graphics_queue)
    }

    fn create_swapchain(
        device: &ash::Device,
        swapchain_loader: &ash::extensions::khr::Swapchain,
        surface: vk::SurfaceKHRm
    ) -> vk::SwapchainKHR {
        vk::SwapchainKHR::null()
    }

    pub fn draw(&self) {
        
    }
}