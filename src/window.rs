use crate::platform_impl;

pub struct Window {
   pub(crate) window: platform_impl::Window,
   pub(crate) props: platform_impl::Property,
}

impl Default for Window {
   fn default() -> Self {
      Window{
         window: platform_impl::Window::new(),
         props: platform_impl::Property::new(),
      }
   }
}

impl Window {
   pub fn new() -> Self{
      Default::default()
   }

   pub fn run(&self){
      self.window.run();
   }
}