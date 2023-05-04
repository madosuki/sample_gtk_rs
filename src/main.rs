#![feature(local_key_cell_methods)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::thread;
use std::fs::File;
use std::fs::metadata;
use std::io::Read;
use std::convert::TryInto;

use gio::{prelude::*, ApplicationFlags};
use gtk::prelude::*;
use gtk::prelude::{ GridExt };
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

        let grid = gtk::Grid::new();
        self.main_window.add(&grid);
        
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        grid.attach(&button, 0, 0, 100, 100);

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
        let path = 
            if path_string.len() > 0 {
                Some(std::path::Path::new(&path_string))
            } else {
                None
            };

        if path.is_some() {
            let path_unwraped = path.unwrap();

            if let Some(image_container) = generate_gtk_image_from_file (path_unwraped) {
                let _scrolled = gtk::ScrolledWindow::builder().child(&image_container.image_object).build();
                grid.attach(&_scrolled, 0, 200, image_container.pixbuf_data.width(), image_container.pixbuf_data.height());
            }
            
        }

        self.main_window.show_all();
    }
}

struct ImageContainer {
    image_object: gtk::Image,
    pixbuf_data: gdk_pixbuf::Pixbuf,
}

fn generate_gtk_image_from_file(file_path: &std::path::Path) -> Option<ImageContainer> {
    let md = metadata(file_path).unwrap();
    
    if md.is_file() {

        let _image = gtk::Image::new();
        
        // using pixbuf loader pattern
        let mut f = File::open(file_path).unwrap();
        let mut buf: Vec<u8> = vec!();
        let _ = f.read_to_end(&mut buf);
        let pixbuf_loader = gdk_pixbuf::PixbufLoader::new();
        let _ = pixbuf_loader.write(&buf);
        let pixbuf_data = pixbuf_loader.pixbuf().unwrap();
        let _ = pixbuf_loader.close();
        _image.set_from_pixbuf(Some(&pixbuf_data));

        let image_container = ImageContainer {
            image_object: _image,
            pixbuf_data,
        };


        return Some(image_container);
    }

    None
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
