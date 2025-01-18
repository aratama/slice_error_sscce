```sh
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target\debug\bevy-test.exe`
2025-01-18T21:57:20.479649Z  INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Windows 11 Home", kernel: "26100", cpu: "13th Gen Intel(R) Core(TM) i9-13900H", core_count: "14", memory: "31.6 GiB" }
2025-01-18T21:57:20.584084Z  INFO bevy_render::renderer: AdapterInfo { name: "Intel(R) Iris(R) Xe Graphics", vendor: 32902, device: 42912, device_type: IntegratedGpu, driver: "Intel Corporation", driver_info: "101.5768", backend: Vulkan }
2025-01-18T21:57:21.320746Z  INFO bevy_winit::system: Creating new window "App" (0v1#4294967296)
thread 'main' panicked at C:\Users\USERNAME\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_ecs-0.15.0\src\system\commands\mod.rs:1982:13:
error[B0003]: C:\Users\USERNAME\Documents\dev\bevy_aseprite_ultra\src\slice.rs:99:28: Could not insert a bundle (of type `bevy_aseprite_ultra::FullyLoaded`) for entity 10v1#4294967306 because it doesn't exist in this World. See: https://bevyengine.org/learn/errors/b0003
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Encountered a panic when applying buffers for system `bevy_aseprite_ultra::slice::update_aseprite_sprite_slice`!
Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
error: process didn't exit successfully: `target\debug\bevy-test.exe` (exit code: 101)
```
