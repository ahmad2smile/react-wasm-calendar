extern crate stdweb;

mod calendar;
mod canvas;
mod date_button;

use calendar::Calendar;
use canvas::Canvas;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	stdweb::initialize();

	let canvas = Canvas::new("#calendarCanvas", 20, 20);

	let calendar = Rc::new(RefCell::new(Calendar::new(20, 20)));
	calendar.borrow().draw(&canvas);

	stdweb::event_loop();
}
