use std::fs;
use zed_extension_api::{self as zed, GithubReleaseOptions, Result};

struct ArduinoExtension {
    cached_binary_path: Option<String>,
}

impl ArduinoExtension {
    fn ensure_arduino_cli(&self) -> Result<String> {
        let cli_bin_name = "arduino-cli";
        let bin_dir = "bin";
        let cli_path = format!("{}/{}", bin_dir, cli_bin_name);

        let current_dir =
            std::env::current_dir().map_err(|e| format!("failed to get current dir: {}", e))?;
        let absolute_cli_path = current_dir.join(&cli_path).to_string_lossy().to_string();

        if fs::metadata(&cli_path)
            .map(|m| m.is_file())
            .unwrap_or(false)
        {
            return Ok(absolute_cli_path);
        }

        let release = zed::latest_github_release(
            "arduino/arduino-cli",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, _arch) = zed::current_platform();
        let asset = release
            .assets
            .iter()
            .find(|a| {
                let name = a.name.to_lowercase();
                name.contains(match platform {
                    zed::Os::Mac => "macos",
                    zed::Os::Linux => "linux",
                    zed::Os::Windows => "windows",
                }) && name.contains("64bit")
            })
            .ok_or_else(|| "No arduino-cli asset found".to_string())?;

        fs::create_dir_all(bin_dir).map_err(|e| format!("failed to create bin dir: {}", e))?;

        zed::download_file(
            &asset.download_url,
            bin_dir,
            zed::DownloadedFileType::GzipTar,
        )
        .map_err(|e| format!("failed to download arduino-cli: {}", e))?;

        zed::make_file_executable(&cli_path)?;

        Ok(absolute_cli_path)
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "arduino/arduino-language-server",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, _arch) = zed::current_platform();
        let asset = release
            .assets
            .iter()
            .find(|a| {
                a.name.contains(match platform {
                    zed::Os::Mac => "macOS",
                    zed::Os::Linux => "Linux",
                    zed::Os::Windows => "Windows",
                }) && a.name.contains("64bit")
            })
            .ok_or_else(|| "No LSP asset found".to_string())?;

        let version_dir = format!("arduino-lsp-{}", release.version);
        let binary_path = format!("{}/arduino-language-server", version_dir);

        if !fs::metadata(&binary_path)
            .map(|m| m.is_file())
            .unwrap_or(false)
        {
            fs::create_dir_all(&version_dir).map_err(|e| format!("failed to create dir: {}", e))?;
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )?;
            zed::make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for ArduinoExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id)?;
        let cli_path = self.ensure_arduino_cli()?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![
                "-cli".into(),
                cli_path.into(),
                "-clangd".into(),
                "clangd".into(),
                "-fqbn".into(),
                "arduino:avr:uno".into(),
            ],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(ArduinoExtension);
