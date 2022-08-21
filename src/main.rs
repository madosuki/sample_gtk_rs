#![feature(local_key_cell_methods)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::thread;

use gio::{prelude::*, ApplicationFlags};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

thread_local! {
    static FILE_PATH: RefCell<Option<String>> = RefCell::new(Option::None);
}

fn on_activate(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("First GTK+ Program");
    window.set_default_size(1024, 768);
    
    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Clicked!");
    });
    // window.add(&button);
    //

    let _image = gtk::Image::new();
    let path_string =
        FILE_PATH.with_borrow(|x| -> String {
            match x {
                Some(v) => {
                    v.clone()
                },
                _ => "".to_owned()
            }
        })
    ;
    println!("{}", &path_string);
    let path = 
        if path_string.len() > 0 {
            Some(std::path::Path::new(&path_string))
        } else {
            None
        };
    _image.set_from_file(path);
    let _scrolled = gtk::ScrolledWindow::builder().child(&_image).build();

    window.add(&_scrolled);
    // window.set_child(Some(&_scrolled));

    window.show_all();
}

fn local_options_handler(_app: &Application, _options: &glib::VariantDict) -> i32 {
    println!("handler now!");

    match _options.lookup_value("file", Option::None) {
        Some(v) => {
            let s = v.data().iter().map(|&i| i as char).collect::<String>().trim_matches(char::from(0)).to_owned();
            FILE_PATH.with(|x| {
                *x.borrow_mut() = Some(s);
            })
        },
        _ => {
            println!("not found");
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
