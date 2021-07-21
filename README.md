# Kaldera
Lightweight realtime raytracing renderer with Vulkan, written in Rust. 

![image](https://user-images.githubusercontent.com/25946200/90334341-fa7b5100-e007-11ea-9c0a-a626e58c77de.png)

## Requirements
You will need the following environment.

- Ubuntu 20.04 LTS or a related distribution
- Vulkan 1.2 or above
- X Window System (libxcb)
- VK_KHR_ray_tracing_pipeline supported hardware and display driver such as
    - NVIDIA GeForce RTX 2070
    - NVIDIA Display Driver 470.57.02
- Rust 1.42.0 or above

## Build


```
git clone https://github.com/ogukei/kaldera.git
cd kaldera
git submodule update --init 
cargo run --release
```
