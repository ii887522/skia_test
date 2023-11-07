use super::{Unit, View};
use crate::{models::Box2D, Context};
use sdl2::{event::Event, image::LoadSurface, mixer, surface::Surface, video::GLProfile};
use skia_safe::{
  gpu::{
    backend_render_targets,
    gl::{Format, FramebufferInfo},
    surfaces, DirectContext, SurfaceOrigin,
  },
  Canvas, Color, ColorType,
};
use std::time::Instant;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

pub fn run(mut app: App<impl View>) {
  // Preconditions
  assert_ne!(app.size.0, 0, "size.0 must be a positive integer");
  assert_ne!(app.size.1, 0, "size.1 must be a positive integer");

  // Fix blurry windows
  #[cfg(windows)]
  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).unwrap();
  }

  let sdl = sdl2::init().unwrap();
  let mut context = Context::new();

  if app.play_audio {
    // Initialize audio engine
    mixer::open_audio(44100, mixer::DEFAULT_FORMAT, 2, 256).unwrap();
    context.init_audio()
  }

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
    .window(app.title, app.size.0, app.size.1)
    .opengl()
    .allow_highdpi()
    .position_centered()
    .build()
    .unwrap();
  window.set_icon(Surface::from_file("assets/images/favicon.png").unwrap());

  // Initialize OpenGL context and make it current in this thread
  let _gl_ctx = window.gl_create_context().unwrap();
  gl::load_with(|name| vid_subsys.gl_get_proc_address(name) as *const _);

  // Enable VSync
  vid_subsys.gl_set_swap_interval(1).unwrap();

  // Initialize Skia engine on top of the OpenGL context
  let mut gr_ctx = DirectContext::new_gl(None, None).unwrap();
  let render_target = backend_render_targets::make_gl(
    (app.size.0 as _, app.size.1 as _),
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
  let mut prev = Instant::now();
  'running: loop {
    // Input
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event {
        break 'running;
      }

      app.on_event(&mut context, &event);
    }

    // Before process
    let now = Instant::now();
    let mut dt_left = (now - prev).as_secs_f32();
    prev = now;
    let mut ticks_left = 8; // 8 max ticks per frame

    // Process
    while ticks_left > 0 && dt_left > 0f32 {
      let dt = dt_left.min(1f32 / 120f32); // 120 ticks per second
      app.tick(&mut context, dt);
      dt_left -= dt;
      ticks_left -= 1;
    }

    // Output
    app.draw(
      &mut context,
      canvas,
      Box2D {
        position: (0f32, 0f32),
        size: (app.size.0 as _, app.size.1 as _),
      },
    );
    gr_ctx.flush_and_submit();
    window.gl_swap_window();
  }

  // Cleanup
  if app.play_audio {
    mixer::close_audio();
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct App<'a, Child = Unit> {
  pub title: &'a str,
  pub size: (u32, u32),
  pub color: Option<Color>,
  pub play_audio: bool,
  pub child: Child,
}

impl<Child: View> View for App<'_, Child> {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    self.child.on_event(context, event);
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    self.child.tick(context, dt);
  }

  fn draw(&self, context: &Context, canvas: &Canvas, constraint: Box2D) {
    canvas.clear(self.color.unwrap_or(Color::BLACK));
    self.child.draw(context, canvas, constraint);
  }
}
