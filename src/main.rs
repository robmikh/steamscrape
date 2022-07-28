use scraper::{Html, Selector};

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        panic!()
    }
    let path = &args[0];

    let content = std::fs::read_to_string(path).unwrap();
    let document = Html::parse_document(&content);
    let game_selector = Selector::parse(r#"div[class="gameListRowItem"]"#).unwrap();
    let hours_selector = Selector::parse(r#"h5[class="ellipsis hours_played"]"#).unwrap();

    let mut num_games = 0;
    let mut num_games_played = 0;
    for game in document.select(&game_selector) {
        let hours = game.select(&hours_selector).next().unwrap();

        let hours_text = hours.inner_html();

        if !hours_text.is_empty() {
            num_games_played += 1;
        }

        num_games += 1;
    }

    println!("Num games: {}", num_games);
    println!("Num games played: {}", num_games_played);

    let percentage = (num_games_played as f32 / num_games as f32) * 100.0;
    println!("Percentage of games played: {:.2}%", percentage);
}
