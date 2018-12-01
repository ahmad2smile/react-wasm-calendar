use canvas::Canvas;

pub struct DateButton {
	pub x: u32,
	pub y: u32,
	pub date: String,
	pub date_x: u32,
	pub date_y: u32,
	pub color: String,
	pub selected: bool,
}

impl DateButton {
	pub fn draw(&self, canvas: &Canvas) {
		assert!(self.x < canvas.width);
		assert!(self.y < canvas.height);

		canvas.ctx.set_stroke_style_color(&self.color);
		canvas.ctx.set_fill_style_color(&self.color);

		let x = self.x * canvas.scaled_width;
		let y = self.y * canvas.scaled_height;

		canvas.ctx.begin_path();
		canvas
			.ctx
			.arc(f64::from(x), f64::from(y), 35.0, 0.0, 2.0 * 3.14, false);
		canvas.ctx.set_fill_style_color("black");
		canvas.ctx.set_font("20px sans-serif");
		canvas
			.ctx
			.fill_text(&self.date, f64::from(x), f64::from(y), Some(20.0));
		canvas.ctx.stroke();
	}
}
