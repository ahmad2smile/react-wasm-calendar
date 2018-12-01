use canvas::Canvas;
use date_button::DateButton;

pub struct Calendar {
	month_grid: Vec<DateButton>,
	month_name: String,
	year: u32,
	width: u32,
	height: u32,
}

impl Calendar {
	pub fn new(width: u32, height: u32) -> Calendar {
		let mut month_grid: Vec<DateButton> = Vec::new();

		for row in 2..8 {
			for col in 1..8 {
				let date = row + col;

				month_grid.push(DateButton {
					x: col * 2,
					y: row * 2,
					date: date.to_string(),
					date_x: col * 2,
					date_y: row * 2,
					color: String::from("lightgreen"),
					selected: false,
				});
			}
		}

		Calendar {
			month_grid,
			month_name: String::from("December"),
			year: 2018,
			width,
			height,
		}
	}

	pub fn draw(&self, canvas: &Canvas) {
		canvas.clear_all();

		canvas.ctx.set_fill_style_color("black");
		canvas.ctx.set_font("40px sans-serif");

		let calendar_header = self.month_name.clone() + ", " + &self.year.to_string();

		canvas.text(&calendar_header, 1.0, 2.0, 250, 40, "black");

		for date_button in &self.month_grid {
			DateButton::draw(date_button, canvas);
		}
	}
}
