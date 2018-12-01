use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
	pub canvas: CanvasElement,
	pub ctx: CanvasRenderingContext2d,
	pub scaled_height: u32,
	pub scaled_width: u32,
	pub height: u32,
	pub width: u32,
}

impl Canvas {
	pub fn new(attr_id: &str, height: u32, width: u32) -> Canvas {
		let canvas: CanvasElement = document()
			.query_selector(attr_id)
			.unwrap()
			.unwrap()
			.try_into()
			.unwrap();

		let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

		let scaled_height = canvas.height() / height;
		let scaled_width = canvas.width() / width;

		Canvas {
			canvas,
			ctx,
			scaled_height,
			scaled_width,
			height,
			width,
		}
	}

	pub fn text(&self, text: &str, x: f64, y: f64, width: u32, font_size: u32, color: &str) {
		let x = x * f64::from(self.scaled_width);
		let y = y * f64::from(self.scaled_height);

		self.ctx.set_fill_style_color(color);

		let fonts = font_size.to_string() + "px sans-serif";

		self.ctx.set_font(&fonts);

		self.ctx
			.fill_text(text, f64::from(x), f64::from(y), Some(f64::from(width)));
	}

	pub fn clear_all(&self) {
		self.ctx.set_fill_style_color("white");
		self.ctx.fill_rect(
			0.0,
			0.0,
			f64::from(self.width * self.scaled_width),
			f64::from(self.height * self.scaled_height),
		)
	}
}
