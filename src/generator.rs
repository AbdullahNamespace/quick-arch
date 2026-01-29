use crate::config::{DirectoryItem, FileItem, GenerationStats, ProjectConfig};
use crate::evaluator::{build_context, evaluate_condition};
use crate::ui::{print_step, Status};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct ProjectGenerator {
    config: ProjectConfig,
    context: HashMap<String, String>,
    output_dir: PathBuf,
}

impl ProjectGenerator {
    pub fn new(config_path: &str) -> Result<Self> {
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Could not read config file: {}", config_path))?;

        let config: ProjectConfig =
            serde_json::from_str(&content).with_context(|| "Failed to parse JSON config")?;

        let context = build_context(&config.features)?;
        let output_dir = PathBuf::from(&config.project.name);

        Ok(Self {
            config,
            context,
            output_dir,
        })
    }

    pub fn generate(&mut self, output_arg: Option<&str>) -> Result<GenerationStats> {
        if let Some(out) = output_arg {
            self.output_dir = PathBuf::from(out);
        }

        let project_name = self.config.project.name.clone();
        let output_str = self.output_dir.display().to_string();

        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)?;
        }

        let total_items = self.config.directories.len() + self.config.files.len();
        let mut current_count = 0;

        crate::ui::print_section("Creating Directories");
        let mut created_dirs = 0;
        let mut skipped_dirs = 0;

        for item in &self.config.directories {
            current_count += 1;
            let (path, condition) = match item {
                DirectoryItem::Simple(p) => (p.clone(), None),
                DirectoryItem::Complex(c) => (c.path.clone(), c.condition.clone()),
            };

            let skip = if let Some(cond) = &condition {
                !evaluate_condition(cond, &self.context)?
            } else {
                false
            };

            if skip {
                print_step(
                    current_count,
                    total_items as u32,
                    "Skip Dir",
                    &path,
                    Status::Skip,
                );
                skipped_dirs += 1;
                continue;
            }

            let full_path = self.output_dir.join(&path);
            if !full_path.exists() {
                fs::create_dir_all(&full_path)?;
                print_step(
                    current_count,
                    total_items as u32,
                    "Create Dir",
                    &path,
                    Status::Success,
                );
                created_dirs += 1;
            } else {
                print_step(
                    current_count,
                    total_items as u32,
                    "Exists Dir",
                    &path,
                    Status::Skip,
                );
                skipped_dirs += 1;
            }
        }

        crate::ui::print_section("Creating Files");
        let mut created_files = 0;
        let mut skipped_files = 0;

        for item in &self.config.files {
            current_count += 1;
            let (path, content, condition) = match item {
                FileItem::Simple(p) => (p.clone(), None, None),
                FileItem::Complex(c) => (c.path.clone(), c.content.clone(), c.condition.clone()),
            };

            let skip = if let Some(cond) = &condition {
                !evaluate_condition(cond, &self.context)?
            } else {
                false
            };

            if skip {
                print_step(
                    current_count,
                    total_items as u32,
                    "Skip File",
                    &path,
                    Status::Skip,
                );
                skipped_files += 1;
                continue;
            }

            let full_path = self.output_dir.join(&path);

            if let Some(parent) = full_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            if !full_path.exists() {
                if let Some(text) = content {
                    fs::write(&full_path, text)?;
                } else {
                    fs::File::create(&full_path)?;
                }
                print_step(
                    current_count,
                    total_items as u32,
                    "Create File",
                    &path,
                    Status::Success,
                );
                created_files += 1;
            } else {
                print_step(
                    current_count,
                    total_items as u32,
                    "Exists File",
                    &path,
                    Status::Skip,
                );
                skipped_files += 1;
            }
        }

        let scripts = if self.config.custom_scripts.post_create.is_empty() {
            None
        } else {
            Some(self.config.custom_scripts.post_create.clone())
        };

        Ok(GenerationStats {
            project_name,
            output_dir: output_str,
            dirs_count: created_dirs,
            files_count: created_files,
            skipped_count: skipped_dirs + skipped_files,
            scripts_to_run: scripts,
        })
    }

    pub fn execute_scripts(&self, scripts: &[String]) -> Result<()> {
        for script in scripts {
            crate::ui::print_info(&format!("Running: {}", script));

            let result = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", script])
                    .current_dir(&self.output_dir)
                    .output()
            } else {
                Command::new("sh")
                    .args(["-c", script])
                    .current_dir(&self.output_dir)
                    .output()
            };

            match result {
                Ok(output) => {
                    if !output.status.success() {
                        crate::ui::print_error(&format!(
                            "Script failed: {}",
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                }
                Err(e) => {
                    crate::ui::print_error(&format!("Failed to execute script: {}", e));
                }
            }
        }
        Ok(())
    }
}
