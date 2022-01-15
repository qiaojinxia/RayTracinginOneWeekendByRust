// use speedy2d::color::Color;
// use speedy2d::window::{WindowHandler, WindowHelper};
// use speedy2d::Graphics2D;
// use std::sync::{Arc, Mutex, RwLock};
// use crate::ray::Point3;
// use std::sync::mpsc::Receiver;
//
// pub(crate) struct MyWindowHandler {
//     pub(crate) Buffer:Arc<RwLock<Vec<Point3>>>,
//     pub(crate) TX:Receiver<()>
// }
//
// impl WindowHandler for MyWindowHandler
// {
//     fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
//     {
//         let buffer = self.Buffer.read().unwrap();
//         for _ in 0..10 {
//             let _ = self.TX.recv();
//         }
//         for panic_abort in buffer.iter(){
//             graphics.clear_screen(Color::from_rgb(panic_abort.x as f32, panic_abort.y as f32, panic_abort.z as f32));
//         }
//         // Request that we draw another frame once this one has finished
//         helper.request_redraw();
//     }
//
//     // If desired, on_mouse_move(), on_key_down(), etc...
// }