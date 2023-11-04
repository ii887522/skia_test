#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::{event::Event, image::LoadSurface, surface::Surface, video::GLProfile};
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

fn main() {
  // Fix blurry windows
  #[cfg(windows)]
  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).unwrap();
  }

  let sdl = sdl2::init().unwrap();
  let vid_subsys = sdl.video().unwrap();
  let gl_attr = vid_subsys.gl_attr();
  gl_attr.set_red_size(8);
  gl_attr.set_green_size(8);
  gl_attr.set_blue_size(8);
  gl_attr.set_context_flags().forward_compatible().set();
  gl_attr.set_context_no_error(true);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_version(4, 6);
  gl_attr.set_depth_size(24);

  let mut window = vid_subsys
    .window("Snake", 1600, 900)
    .opengl()
    .allow_highdpi()
    .position_centered()
    .build()
    .unwrap();

  window.set_icon(Surface::from_file("assets/favicon.png").unwrap());

  // Initialize OpenGL context and make it current in this thread
  let _gl_ctx = window.gl_create_context().unwrap();
  gl::load_with(|name| vid_subsys.gl_get_proc_address(name) as *const _);

  // Enable VSync
  vid_subsys.gl_set_swap_interval(1).unwrap();

  // Game loop
  let mut event_pump = sdl.event_pump().unwrap();
  loop {
    // Input
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event {
        return;
      }
    }

    // Output
    window.gl_swap_window();
  }
}
