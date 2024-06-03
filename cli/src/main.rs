mod generators;
mod templates;
mod helpers;

fn main() {
    let current_dir = std::env::current_dir().unwrap();      
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        return;
    }

    let generate_aliases = [
        "g",
        "gen",
        "generate"
    ];

    if generate_aliases.contains(&&args[1][..]) {
        let generators = ["service", "repository", "controller"];
        
        if args.len() < 3 || !generators.contains(&&args[2][..]) {
            return;
        }

        match args[2].as_str() {
            "service" => generators::service(&args, &current_dir),
            "repository" => generators::repository(&args, &current_dir),
            "controller" => generators::controller(&args, &current_dir),
            _ => eprintln!("Invalid argument --{} provided: {}.", args[1], args[2])
        };
    };
}