use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();
    let cat_text = r#"Meow!
    \
     \
       /\_/\
      ( o o )
      =( I )="#;
    siv.add_layer(TextView::new(cat_text));
    siv.run();
}
