pub mod matchs {
  pub fn matchs(prog_lang: &str) {
    // Match
    let objective: &str = match prog_lang {
        "Python" => "Data Science",
        "Rust" => "Systems Programming",
        "JavaScript" => "Web Development",
        "PHP" => "Web Development",
        "Java" => "Android Development",
        "C#" => "Game Development",
        "C++" => "Game Development",
        "C" => "Systems Programming",
        "Go" => "Systems Programming",
        "Kotlin" => "Android Development",
        "Swift" => "iOS Development",
        "TypeScript" => "Web Development",
        "Ruby" => "Web Development",
        "Scala" => "Big Data",
        "Julia" => "Data Science",
        "R" => "Data Science",
        "Haskell" => "Artificial Intelligence",
        "Clojure" => "Artificial Intelligence",
        "Elixir" => "Web Development",
        "Erlang" => "Distributed Systems",
        "Lua" => "Game Development",
        "Perl" => "Web Development",
        "Dart" => "Mobile Development",
        _ => "Unknown", // Default
    };

    // Print result
    println!("Language {} is commonly used for {}", prog_lang, objective);
  }
}