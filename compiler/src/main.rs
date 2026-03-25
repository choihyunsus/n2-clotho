// n2c — .n2 compiler CLI (parsing + validation + contract + query + multi-target codegen)
use std::env;
use std::fs;
use std::path::Path;
use n2_compiler::parser::parse_n2;
use n2_compiler::validator;
use n2_compiler::contract::ContractRuntime;
use n2_compiler::query::N2Registry;
use n2_compiler::codegen::{BackendRegistry, CompilationMeta, extract_meta};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // Handle commands that don't need a file
    if args[1] == "backends" {
        cmd_backends();
        return;
    }

    let (command, filepath) = if args.len() >= 3 {
        (args[1].as_str(), &args[2])
    } else {
        ("parse", &args[1])
    };

    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read file [{}]: {}", filepath, e);
            std::process::exit(1);
        }
    };

    println!("🔧 n2c v{} — Clotho compiler", VERSION);
    println!("📄 File: {}", filepath);
    println!();

    // Step 1: Parse
    let ast = match parse_n2(&source) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    };

    match command {
        "parse" => cmd_parse(&ast),
        "validate" => cmd_validate(&ast),
        "simulate" => cmd_simulate(&ast),
        "query" => cmd_query(&ast, args.get(3).cloned()),
        "compile" => cmd_compile(&ast, filepath, &args[3..]),
        _ => {
            eprintln!("❌ Unknown command: '{}'", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("🔧 n2c v{} — Clotho multi-target compiler", VERSION);
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  n2c <file.n2>                                 Parse + AST JSON output");
    eprintln!("  n2c validate <file.n2>                        Parse + validate + contract check");
    eprintln!("  n2c simulate <file.n2>                        Contract state machine simulation");
    eprintln!("  n2c query <file.n2> \"SELECT * FROM rules\"     SQL query");
    eprintln!("  n2c compile <file.n2> --target <TARGET>       Compile to specific target");
    eprintln!("  n2c compile <file.n2> --target all            Compile to all targets");
    eprintln!("  n2c backends                                  List supported backends");
    eprintln!();
    eprintln!("Targets: rust(.n2rs) | c(.n2c) | cpp(.n2c2) | go(.n2go) | python(.n2py) | ts(.n2ts)");
}

fn cmd_parse(ast: &n2_compiler::ast::N2File) {
    let json = serde_json::to_string_pretty(ast).unwrap();
    println!("✅ Parse success! AST:");
    println!("{}", json);
    print_summary(ast);
}

fn cmd_validate(ast: &n2_compiler::ast::N2File) {
    println!("── Step 1: Parse ✅");
    print_summary(ast);

    println!();
    println!("── Step 2: Schema Validation");
    let errors = validator::validate(ast);
    let error_count = errors.iter()
        .filter(|e| e.severity == validator::Severity::Error)
        .count();
    let warn_count = errors.iter()
        .filter(|e| e.severity == validator::Severity::Warning)
        .count();

    for err in &errors {
        println!("  {}", err);
    }

    if error_count == 0 && warn_count == 0 {
        println!("  ✅ Validation passed! 0 errors, 0 warnings");
    } else {
        println!("  Result: ❌ {} errors, ⚠️ {} warnings", error_count, warn_count);
    }

    println!();
    println!("── Step 3: Contract Check");
    let runtime = ContractRuntime::from_file(ast);

    if runtime.machines.is_empty() {
        println!("  ℹ️ No state machine contracts (skipped)");
    } else {
        println!("{}", runtime.summary().lines()
            .map(|l| format!("  {}", l))
            .collect::<Vec<_>>()
            .join("\n"));

        let violations = runtime.check_integrity();
        if violations.is_empty() {
            println!("  ✅ State machine integrity check passed!");
        } else {
            for v in &violations {
                println!("  {}", v);
            }
        }
    }

    println!();
    if error_count > 0 {
        println!("🚨 Validation failed: {} errors found", error_count);
        std::process::exit(1);
    } else {
        println!("✅ Validation complete: all checks passed!");
    }
}

fn cmd_simulate(ast: &n2_compiler::ast::N2File) {
    println!("── Contract state machine simulation");
    let runtime = ContractRuntime::from_file(ast);

    if runtime.machines.is_empty() {
        println!("  ℹ️ No state machine contracts found");
        std::process::exit(0);
    }

    println!("{}", runtime.summary());
}

fn cmd_query(ast: &n2_compiler::ast::N2File, extra: Option<String>) {
    let registry = N2Registry::from_file(ast);
    println!("{}", registry.summary());
    println!();

    let sql = extra.unwrap_or_else(|| "SELECT * FROM rules".to_string());
    println!("📝 SQL: {}", sql);
    println!();

    match registry.execute_query(&sql) {
        Ok(result) => print!("{}", result),
        Err(e) => {
            eprintln!("❌ Query error: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_compile(ast: &n2_compiler::ast::N2File, filepath: &str, remaining_args: &[String]) {
    // Parse --target from remaining args
    let target = parse_target_arg(remaining_args);

    let registry = BackendRegistry::new();
    let (source_name, source_version) = extract_meta(ast);

    let now = chrono_now();
    let base_meta = CompilationMeta {
        source_name: source_name.clone(),
        source_version: source_version.clone(),
        target: String::new(),
        extension: String::new(),
        compiler_version: format!("n2c v{}", VERSION),
        compiled_at: now,
    };

    let base_path = Path::new(filepath).with_extension("");

    if target == "all" {
        println!("🎯 Compiling to all targets");
        println!();

        let results = registry.compile_all(ast, &base_meta);
        let mut success_count = 0;
        let mut fail_count = 0;

        for (target_name, ext, result) in &results {
            let out_path = format!("{}{}", base_path.display(), ext);
            match result {
                Ok(code) => {
                    fs::write(&out_path, code).unwrap_or_else(|e| {
                        eprintln!("  ❌ {} write failed: {}", out_path, e);
                    });
                    println!("  ✅ {} → {} ({} bytes)", target_name, out_path, code.len());
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("  ❌ {} failed: {}", target_name, e);
                    fail_count += 1;
                }
            }
        }

        println!();
        println!("📊 Result: {} success, {} failed / {} targets",
            success_count, fail_count, results.len());
    } else {
        let mut meta = base_meta;
        meta.target = target.to_string();

        match registry.compile(ast, &target, &meta) {
            Ok(code) => {
                let ext = registry.get(&target)
                    .map(|b| b.file_extension())
                    .unwrap_or(".n2out");
                let out_path = format!("{}{}", base_path.display(), ext);
                fs::write(&out_path, &code).unwrap_or_else(|e| {
                    eprintln!("❌ Write failed: {}", e);
                    std::process::exit(1);
                });
                println!("✅ {} → {} ({} bytes)", target, out_path, code.len());
            }
            Err(e) => {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Parse --target argument from remaining CLI args
/// Supports: "--target=rust", "--target rust", bare "rust"
fn parse_target_arg(args: &[String]) -> String {
    if args.is_empty() {
        return "all".to_string();
    }

    let first = &args[0];

    // --target=rust (joined form)
    if let Some(val) = first.strip_prefix("--target=") {
        return val.to_string();
    }

    // --target rust (split form)
    if first == "--target" {
        return args.get(1).cloned().unwrap_or_else(|| "all".to_string());
    }

    // Bare target name: "rust", "all", etc.
    first.to_string()
}

fn cmd_backends() {
    println!("🔧 n2c v{} — Clotho multi-target compiler", VERSION);
    println!();
    println!("Supported backends:");
    println!();
    let registry = BackendRegistry::new();
    for (name, ext) in registry.list() {
        println!("  {:12} → {}", name, ext);
    }
    println!();
    println!("Total: {} targets supported", registry.list().len());
}

fn print_summary(ast: &n2_compiler::ast::N2File) {
    use n2_compiler::ast::Block;
    let mut counts = [0u32; 8];
    for block in &ast.blocks {
        match block {
            Block::Meta(_) => counts[0] += 1,
            Block::Import(_) => counts[1] += 1,
            Block::Schema(_) => counts[2] += 1,
            Block::Contract(_) => counts[3] += 1,
            Block::Rule(_) => counts[4] += 1,
            Block::Workflow(_) => counts[5] += 1,
            Block::Query(_) => counts[6] += 1,
            Block::Semantic(_) => counts[7] += 1,
        }
    }
    println!("📊 Blocks: @meta:{} @import:{} @schema:{} @contract:{} @rule:{} @workflow:{} @query:{} @semantic:{} | total {}",
        counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6], counts[7],
        ast.blocks.len()
    );
}

/// ISO 8601 timestamp using std::time (no external dependency)
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let mins = (remaining % 3600) / 60;
    let s = remaining % 60;
    let mut y = 1970i64;
    let mut d = days as i64;
    loop {
        let days_in_year = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 366 } else { 365 };
        if d < days_in_year { break; }
        d -= days_in_year;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let month_days = [
        31, if leap { 29 } else { 28 }, 31, 30, 31, 30,
        31, 31, 30, 31, 30, 31
    ];
    let mut m = 0;
    for md in &month_days {
        if d < *md { break; }
        d -= md;
        m += 1;
    }
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, m + 1, d + 1, hours, mins, s)
}
