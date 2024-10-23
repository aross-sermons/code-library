## Vulkan Tutorial Using Ash  
## Basic Rendering Overview  
### Rendering Components  
**Instance** - The entry point into the Vulkan API.  
- Connection between the application and operating system  
**Physical Device** - A GPU or other hardware accelerator.  
- Contains information on the device's capabilities  
**Logical Device** - An abstraction layer that interacts with the physical device.  
- Can access physical device's features  
- Primary interface for communicating with graphics device  
**Swapchain** - A collection of images that the graphics device renders to the display.  
- Handles exchange of images between rendering and display  
- Manages multiple buffers  
**Pipeline** - Defines the steps that the graphics device follows to render a frame.  
- Consists of multiple stages  
**Command Buffers** - Used to record drawing commands, state changes, and resource transitions.  
**Surface** - 
**Window** - 
## Components  
A list of components needed to interact with the GPU through Vulkan/Ash.  
### Instance  
**Purpose** - Connects the application to the Vulkan library.  
**Has**  
- Application Information - `vk::ApplicationInfo`  
    - This is essentially metadata about the application  
- Instance Creation Information - `ck::InstanceCreateInfo`  
    - The information needed to create the instance  
    - Includes extensions and validation layers  
**Creation Steps**  
1. Load Vulkan Entry - `ash::Entry`  
    - Load the Vulkan library to build from  
2. Define Application Info - `vk::ApplicationInfo`  
    - `application_name` - The name of the application  
    - `applcation_version` - The version of the application  
    - `engine_name` - The name of the engine used (if any)  
    - `engine_version` - The version of the engine  
    - `api_version` - The Vulkan API version to use  
3. Check Extensions - `ash::extensions::*`  
    - Extensions used for rendering  
    - Use `entry.enumerate_instance_extension_properties` for a list of available extensions  
4. Validation Layers  
    - Used for debugging  
5. Define Instance Create Info - `vk::InstanceCreateInfo::builder()`  
6. Create Vulkan Instance - `entry.create_instance()`  
### Physical Device  
**Purpose** - Represents a physical compute device (GPU, integrated graphics, etc.).  
**Has**  
- Queue Families  
    - Different physical devices may support multiple queues for different graphics operations  
**Steps**  
1. Query Available Physical Devices - `instance.enumerate_physical_devices()`  
    - Get a list of all available graphpics devices  
2. Select Physical Device  
3. Check for Queue Families  `instance.get_physical_device_queue_family_porperties(physical_device)`  
### Logical Device  
**Purpose** - The software interface used to interact with the physical device.  
**Has**  
- Device Queues  
    - Where commands are submitted for execution  
    - Each queue corresponds to a queue family  
- Device Features  
    - Enable device specific features, like geometry shaders or tesselation  
- Extentions  
    - Similar to the instance extensions  
**Steps**  
1. Select Queue Families  
    - Ensure the graphics device supports the queue families to be used  
2. Create the Logical Device - `instance.create_device()`  
    - Uses a physical device and a device create info  
### Swapchain  
**Purpose** - A series of images or buffers that are rendered by the graphics device.  
**Has**  
- Images  
- Surface  
    - The abstraction of the native window that the swapchain presents images to  
- Image Format and Extent  
    - Format - Color, depth, and layout of swapchaing images  
    - Extent - Resolution  
**Steps**  
1. Create a Surface  
    - Covered in the Surface section below  
2. Query Swapchain Support  
    - Check the physical device to see what formats and present nodes it supports  
3. Choose Image Format, Present Mode, Extent  
    - Based on physical device's capabilities and window size  
4. Create the Swapchain - `device.create_swapchain_khr()`  
5. Retrieve Swapchain Images  
    - Get the actual image handles from the swapchain for rendering  
### Pipeline  
**Purpose** - 
### Command Buffers  
**Purpose** - 
### Surface
**Purpose** - 
### Window  
**Purpose** - 


