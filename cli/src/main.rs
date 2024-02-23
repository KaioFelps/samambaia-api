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
        let generators = ["service", "repository"];
        
        if args.len() < 3 || !generators.contains(&&args[2][..]) {
            return;
        }

        if args[2] == "service" {
            generators::service(&args, &current_dir);
        }

        if args[2] == "repository" {
            generators::repository(&args, &current_dir);
        }
    };
}