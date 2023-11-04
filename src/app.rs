use sdl2::{event::Event, image::LoadSurface, surface::Surface, video::GLProfile};
use skia_safe::{
  gpu::{
    backend_render_targets,
    gl::{Format, FramebufferInfo},
    surfaces, DirectContext, SurfaceOrigin,
  },
  Canvas, ColorType,
};
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

pub fn run(title: &str, width: u32, height: u32, mut on_draw: impl FnMut(&Canvas, u32, u32)) {
  // Fix blurry windows
  #[cfg(windows)]
  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).unwrap();
  }

  let sdl = sdl2::init().unwrap();
  let vid_subsys = sdl.video().unwrap();

  // Configure OpenGL attributes
  let gl_attr = vid_subsys.gl_attr();
  gl_attr.set_red_size(8);
  gl_attr.set_green_size(8);
  gl_attr.set_blue_size(8);
  gl_attr.set_context_flags().forward_compatible().set();
  gl_attr.set_context_no_error(true);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_version(4, 6);
  gl_attr.set_depth_size(24);

  // Prepare a window
  let mut window = vid_subsys
    .window(title, width, height)
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

  // Initialize Skia engine on top of the OpenGL context
  let mut gr_ctx = DirectContext::new_gl(None, None).unwrap();
  let render_target = backend_render_targets::make_gl(
    (width as _, height as _),
    0,
    8,
    FramebufferInfo {
      fboid: 0,
      format: Format::RGBA8.into(),
      ..Default::default()
    },
  );
  let mut surface = surfaces::wrap_backend_render_target(
    &mut gr_ctx,
    &render_target,
    SurfaceOrigin::BottomLeft,
    ColorType::RGBA8888,
    None,
    None,
  )
  .unwrap();

  // Get the canvas from the Skia engine to start drawing and have fun!
  let canvas = surface.canvas();

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
    on_draw(canvas, width, height);
    gr_ctx.flush_and_submit();
    window.gl_swap_window();
  }
}
