use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::time::{SystemTime, UNIX_EPOCH};

fn usage(program: &str) -> ! {
    eprintln!("usage: {} <input.nem> [program args ...]", program);
    std::process::exit(1);
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let Ok(metadata) = fs::metadata(path) else {
            return false;
        };
        let mode = metadata.permissions().mode();
        return (mode & 0o111) != 0;
    }

    #[cfg(not(unix))]
    {
        true
    }
}

fn find_nemc() -> Result<PathBuf, String> {
    if let Ok(value) = env::var("NEMC") {
        let candidate = PathBuf::from(value);
        if !is_executable(&candidate) {
            return Err(format!(
                "could not find configured `nemc` at {}\n\n`nem` was told to use NEMC={}, but the configured compiler is missing or not executable.",
                candidate.display(),
                candidate.display()
            ));
        }
        return Ok(candidate);
    }

    if let Ok(value) = env::var("PATH") {
        for entry in env::split_paths(&value) {
            let candidate = entry.join("nemc");
            if is_executable(&candidate) {
                return Ok(candidate);
            }
        }
    }

    Err("could not find `nemc`\n\n`nem` requires the NEM compiler to be installed and available in PATH.\n\nSet NEMC to explicitly specify the compiler path:\n    NEMC=/path/to/nemc nem program.nem".to_string())
}

fn is_nem_source(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()).map(|ext| ext == "nem").unwrap_or(false)
}

fn temp_dir(prefix: &str) -> io::Result<PathBuf> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("{}-{}", prefix, nanos));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn compile_file(nemc: &Path, source: &Path, output_dir: &Path) -> Result<PathBuf, String> {
    let generated_c = output_dir.join("program.c");
    let output = output_dir.join("program");

    let status = Command::new(nemc)
        .arg(source)
        .arg(&generated_c)
        .status()
        .map_err(|err| format!("failed to run nemc: {}", err))?;

    if !status.success() {
        return Err(format!("nemc failed with exit code: {}", status.code().unwrap_or(-1)));
    }

    if !generated_c.exists() {
        return Err("nemc did not produce C output".to_string());
    }

    let compile_status = Command::new("cc")
        .arg(&generated_c)
        .arg("-o")
        .arg(&output)
        .status()
        .map_err(|err| format!("failed to compile generated C: {}", err))?;

    if !compile_status.success() {
        return Err(format!("C compilation failed with exit code: {}", compile_status.code().unwrap_or(-1)));
    }

    if !output.exists() {
        return Err("compiler did not produce an executable".to_string());
    }

    Ok(output)
}

fn run_program(path: &Path, argv: &[String]) -> ExitStatus {
    let mut command = Command::new(path);
    command.args(argv);
    command.status().unwrap_or_else(|err| {
        eprintln!("failed to execute program: {}", err);
        std::process::exit(1);
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
    }

    let input = Path::new(&args[1]);
    if !input.exists() {
        eprintln!("error: input file not found: {}", input.display());
        std::process::exit(1);
    }
    if !is_nem_source(input) {
        eprintln!("error: expected a .nem source file: {}", input.display());
        std::process::exit(1);
    }

    let nemc = match find_nemc() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    let temp = match temp_dir("nem-run") {
        Ok(path) => path,
        Err(err) => {
            eprintln!("error: failed to create temporary directory: {}", err);
            std::process::exit(1);
        }
    };

    let executable = match compile_file(&nemc, input, &temp) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("error: {}", err);
            let _ = fs::remove_dir_all(&temp);
            std::process::exit(1);
        }
    };

    let mut perms = fs::metadata(&executable).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&executable, perms).unwrap();

    let program_args = args.iter().skip(2).cloned().collect::<Vec<_>>();
    let status = run_program(&executable, &program_args);

    let _ = fs::remove_dir_all(&temp);

    match status.code() {
        Some(code) => std::process::exit(code),
        None => std::process::exit(1),
    }
}
