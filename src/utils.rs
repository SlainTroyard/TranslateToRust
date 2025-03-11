use anyhow::{Context, Result};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

/// Generate a path for a specific problem in the FuzzForLeetcode project
pub fn get_problem_path(
    contest: u32,
    problem: u32,
    language: &str,
    path_type: &str,
) -> PathBuf {
    let problem_name = format!("weekly_contest_{}_p{}", contest, problem);
    
    match path_type {
        "source" => {
            let ext = if language == "CPP" { "cpp" } else { "c" };
            PathBuf::from(format!("../FuzzForLeetcode/C_CPP/{}/src/{}.{}", language, problem_name, ext))
        },
        "outputs" => {
            PathBuf::from(format!("../FuzzForLeetcode/fuzz_outputs/{}/{}/outputs", language, problem_name))
        },
        "examples" => {
            PathBuf::from(format!("../FuzzForLeetcode/fuzz_outputs/{}/{}/examples", language, problem_name))
        },
        _ => PathBuf::new(),
    }
}

/// List all available problems in the FuzzForLeetcode project
pub fn list_available_problems() -> Result<Vec<(u32, u32, String)>> {
    let mut problems = Vec::new();
    
    // Look in the CPP source directory
    let cpp_dir = PathBuf::from("../FuzzForLeetcode/C_CPP/CPP/src");
    if cpp_dir.exists() {
        for entry in fs::read_dir(&cpp_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy().to_string();
                
                // Parse weekly_contest_XXX_pY.cpp files
                if file_name.starts_with("weekly_contest_") && file_name.ends_with(".cpp") {
                    let parts: Vec<&str> = file_name.split('_').collect();
                    if parts.len() >= 4 {
                        if let Ok(contest) = parts[2].parse::<u32>() {
                            if let Some(problem_str) = parts[3].strip_prefix('p') {
                                if let Some(problem_str) = problem_str.strip_suffix(".cpp") {
                                    if let Ok(problem) = problem_str.parse::<u32>() {
                                        problems.push((contest, problem, "CPP".to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Look in the C source directory
    let c_dir = PathBuf::from("../FuzzForLeetcode/C_CPP/C/src");
    if c_dir.exists() {
        for entry in fs::read_dir(&c_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy().to_string();
                
                // Parse weekly_contest_XXX_pY.c files
                if file_name.starts_with("weekly_contest_") && file_name.ends_with(".c") {
                    let parts: Vec<&str> = file_name.split('_').collect();
                    if parts.len() >= 4 {
                        if let Ok(contest) = parts[2].parse::<u32>() {
                            if let Some(problem_str) = parts[3].strip_prefix('p') {
                                if let Some(problem_str) = problem_str.strip_suffix(".c") {
                                    if let Ok(problem) = problem_str.parse::<u32>() {
                                        problems.push((contest, problem, "C".to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    info!("Found {} problems in the FuzzForLeetcode project", problems.len());
    Ok(problems)
}

/// Create and ensure a directory exists
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
} 