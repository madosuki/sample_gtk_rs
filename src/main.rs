#![feature(local_key_cell_methods)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::thread;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

use gio::{prelude::*, ApplicationFlags};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use glib;
use gdk_pixbuf;

thread_local! {
    static FILE_PATH: RefCell<Option<String>> = RefCell::new(Option::None);
}

struct MainWindow {
    main_window: ApplicationWindow,
}

impl MainWindow {
    fn init(&self) {
        self.main_window.set_title("First GTK+ Program");
        self.main_window.set_default_size(1024, 768);
        
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        // window.add(&button);
        //

        let _image = gtk::Image::new();
        let path_string: String =
            FILE_PATH.with_borrow(|x| -> String {
                match x {
                    Some(v) => {
                        v.clone()
                    },
                    _ => "".to_owned()
                }
            });
        
        println!("path: {}\nbytes: {:?}", &path_string, &path_string.as_bytes());
        // path_string.pop();
        let path = 
            if path_string.len() > 0 {
                Some(std::path::Path::new(&path_string))
            } else {
                None
            };

        if path.is_some() {
            // using pixbuf loader pattern
            let mut f = File::open(path.unwrap()).unwrap();
            let mut buf: Vec<u8> = vec!();
            let _ = f.read_to_end(&mut buf);
            let pixbuf_loader = gdk_pixbuf::PixbufLoader::new();
            let _ = pixbuf_loader.write(&buf);
            let pixbuf_data = pixbuf_loader.pixbuf().unwrap();
            let _ = pixbuf_loader.close();
            _image.set_from_pixbuf(Some(&pixbuf_data));

            // using set_from_file pattern
            // _image.set_from_file(Some(path));

            // _image.set_from_file(path);
            let _scrolled = gtk::ScrolledWindow::builder().child(&_image).build();

            self.main_window.add(&_scrolled);
        }
        // window.set_child(Some(&_scrolled));

        self.main_window.show_all();
        
    }
}

fn on_activate(app: &gtk::Application) {
    let main = MainWindow { main_window: ApplicationWindow::new(app) };
    main.init();
}

fn local_options_handler(_app: &Application, _options: &glib::VariantDict) -> i32 {
    println!("handler now!");

    match _options.lookup_value("file", Option::None) {
        Some(v) => {
            // let s = v.data().iter().map(|&i| i as char).collect::<String>().trim_matches(char::from(0)).to_owned();
            let mut a = v.data().to_vec();
            a.retain(|&x| x != 0);
            let result = std::str::from_utf8(&a).unwrap().to_owned();

            FILE_PATH.with(|x| {
                *x.borrow_mut() = Some(result);
            })
        },
        _ => {
            println!("option not found");
        }
    }
    -1
}

fn main() {
    let application = Application::builder().application_id("com.github.gtk-rs.examples.basic").build();
    let short_name = glib::Char::from(b'f');
    application.add_main_option(
        "file",
        short_name, 
        glib::OptionFlags::IN_MAIN, 
        glib::OptionArg::Filename,
        "descroption", 
        Some("arg description"));
    
    application.connect_activate(on_activate);
    application.connect_handle_local_options(local_options_handler);
    application.run();
}
