use cursive::views::TextView;

fn main() {
	let mut app = cursive::default();
	
	app.load_toml(include_str!("./theme.toml")).unwrap();

	app.add_global_callback('q', |s| s.quit());

	app.add_fullscreen_layer(TextView::new("Hello cursive! Press <q> to quit."));

	app.run();
}