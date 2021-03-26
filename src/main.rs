#![allow(unused)]
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*};
use plotters::prelude::*;
mod myuifile;
use rand::prelude::*;
use plotters_backend::{BackendColor, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{self, TryRecvError};
// use std::sync::mpsc;
use std::time::Instant;


fn draw_chart<DB: DrawingBackend>(
    b: DrawingArea<DB, plotters::coord::Shift>, rx_arr: Vec<(f64, f64)>,
) -> Result<(), Box<dyn Error>>
where
    DB::ErrorType: 'static,
{
    let mut chart = ChartBuilder::on(&b)
        .margin(0)
        // .caption("Sine and Cosine", ("sans-serif", (10).percent_height()))
        // .set_label_area_size(LabelAreaPosition::Left, (5i32).percent_width())
        // .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
        .build_cartesian_2d(0.0..1000.0, 0.0..1.0)?;

    // chart
        // .configure_mesh()
        // .disable_x_mesh()
        // .disable_y_mesh()
        // .draw()?;

    chart.draw_series(LineSeries::new(
        rx_arr,
        &BLACK,
    ))?;

    b.present()?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////

 fn startclicked(a: &App, f: &Frame, w: &Window, abort: &Arc<Mutex<i32>>) {

     *abort.lock().unwrap() = 0;

     let mut a_clone = a.clone();
     let mut f_clone = f.clone();
     let mut w_clone = w.clone();
     let mut abort_clone = abort.clone();

     let (tx, rx) = mpsc::channel();
     let _ncol = 1000; // number of points to measure

     thread::spawn(move || {
         loop {
             let abort = abort_clone.lock().unwrap();
             if *abort == 0 {
                let mut rng = rand::thread_rng();
                let mut tx_arr: Vec<(f64,f64)> = Vec::with_capacity(_ncol as usize);
                for i in 0.._ncol { // (0..=1000).map(|x| x as f64 / 1.0).map(|x| (x, rng.gen()))
                    tx_arr.push((i as f64, rng.gen()));
                    // println!("{:?}", tx_arr[i]);
                }
                thread::sleep(Duration::from_micros(1000)); // time to get the datas
                tx.send(tx_arr).unwrap(); // send (emit) 'i' to channel, receiver will be run on the main thread
            }
            else {
                break;
            }
         }
     });


     loop {
         match rx.recv() {
             Ok(_) => {
                 let rx_array = rx.recv().unwrap();
                 let mut buf1 = String::from("");
                 let mut _x:i32 = f.width() - 20;
                 let mut _y:i32 = f.height() - 20;
                 // println!("{}, {}", x as u32, y as u32);+

                 if _x>0 && _y>0 {
                     let now = Instant::now();
                     let b = SVGBackend::with_string(&mut buf1, (_x as u32, _y as u32)).into_drawing_area();
                     // b.fill(&BLACK);
                     draw_chart(b, rx_array);

                     let img = image::SvgImage::from_data(&buf1).unwrap();
                     f_clone.set_image(Some(img));
                     w_clone.redraw();
                     a_clone.wait();

                     let elapsed = now.elapsed();
                     println!("Elapsed: {:?}", elapsed);
                  }
            }
            Err(_) => { break; }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn stopclicked(abort: &Arc<Mutex<i32>>){
    *abort.lock().unwrap() = 1; // this works --> let mut abort = abort.lock().unwrap(); *abort = 1; // set "abort" value to 1
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // GUI
    let app = app::App::default(); // Base, Gtk, Gleam, Plastic
    let mut ui = myuifile::UserInterface::make_window();
    let abort = Arc::new(Mutex::new(0));

    let mut frame_clone = ui.frame.clone();
    let mut win_clone = ui.win.clone();
    let abort_clone = abort.clone();
    ui.butStart.set_callback(move || {
        println!("START");
        startclicked(&app, &frame_clone, &win_clone, &abort_clone);
    });

    let abort_clone = abort.clone();
    ui.butStop.set_callback(move || {
        println!("STOP");
        stopclicked(&abort_clone);
    });

   ui.win.end();
   ui.win.show();
   app.run().unwrap();
   Ok(())
}















// // src/main.rs
// use fltk::*;
// use fltk::{app::*, window::*};
// use plotters::prelude::*;
// mod myuifile;
// use rand::prelude::*;
// use plotters_backend::{
//     BackendColor, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
// };
// use std::error::Error;
// use std::sync::{Arc, Mutex};
// use std::time::Duration;
// use std::{thread, time};
//
// fn draw_chart<DB: DrawingBackend>(
//     b: DrawingArea<DB, plotters::coord::Shift>,
// ) -> Result<(), Box<dyn Error>>
// where
//     DB::ErrorType: 'static,
// {
//     let mut rng = rand::thread_rng();
//     let mut chart = ChartBuilder::on(&b)
//         .margin(1)
//         // .caption("Sine and Cosine", ("sans-serif", (10).percent_height()))
//         // .set_label_area_size(LabelAreaPosition::Left, (5i32).percent_width())
//         // .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
//         .build_cartesian_2d(0.0..1000.0, 0.0..1.0)?;
//
//     chart
//         .configure_mesh()
//         .disable_x_mesh()
//         .disable_y_mesh()
//         .draw()?;
//
//     chart.draw_series(LineSeries::new(
//         (0..=1000).map(|x| x as f64 / 1.0).map(|x| (x, rng.gen())),
//         &RED,
//     ))?;
//
//     b.present()?;
//
//     Ok(())
// }
//
// ///////////////////////////////////////////////////////////////////////////////////////////////////
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     // GUI
//     let app = App::default(); // Base, Gtk, Gleam, Plastic
//     let mut ui = myuifile::UserInterface::make_window();
//     let abort = Arc::new(Mutex::new(0));
//     // let mut rng = rand::thread_rng();
//
//     ui.butStart.set_callback(move || {
//         println!("START");
//     });
//
//     ui.butStop.set_callback(move || {
//         println!("STOP");
//     });
//
//    ui.win.end();
//    ui.win.show_with_env_args();
//
//    // Drawing
//     while app.wait()  {
//         let mut buf1 = String::from("");
//         let mut _x:i32 = ui.boxPlot.width() - 20;
//         let mut _y:i32 = ui.boxPlot.height() - 20;
//         // println!("{}, {}", x as u32, y as u32);+
//         if _x>0 && _y>0 {
//             let b = SVGBackend::with_string(&mut buf1, (_x as u32, _y as u32)).into_drawing_area();
//             b.fill(&WHITE)?;
//             draw_chart(b)?;
//
//             let img = image::SvgImage::from_data(&buf1).unwrap();
//             ui.boxPlot.set_image(Some(img));
//             ui.win.redraw();
//         }
//         // ui.win.redraw();
//     }
//
//     Ok(())
// }
