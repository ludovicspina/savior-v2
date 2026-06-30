use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseProduct {
    pub name: String,
    pub description: String,
    pub license_status: u32,
    pub status_label: String,
    pub partial_product_key: Option<String>,
    pub grace_period_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsLicenseInfo {
    pub activated: bool,
    pub products: Vec<LicenseProduct>,
}

const LICENSE_STATUS_SCRIPT: &str = r#"
$products = Get-CimInstance SoftwareLicensingProduct |
  Where-Object { $_.PartialProductKey } |
  ForEach-Object {
    [PSCustomObject]@{
      name = $_.Name
      description = $_.Description
      licenseStatus = [uint32]$_.LicenseStatus
      statusLabel = switch ($_.LicenseStatus) {
        0 { 'Non licencié' }
        1 { 'Licencié' }
        2 { 'Période de grâce (OOB)' }
        3 { 'Période de grâce (OOT)' }
        4 { 'Non authentique' }
        5 { 'Notification' }
        6 { 'Grâce étendue' }
        default { "Code $($_.LicenseStatus)" }
      }
      partialProductKey = $_.PartialProductKey
      gracePeriodRemaining = [uint32]$_.GracePeriodRemaining
    }
  }
[PSCustomObject]@{
  activated = [bool]($products | Where-Object { $_.licenseStatus -eq 1 })
  products = @($products)
} | ConvertTo-Json -Depth 4 -Compress
"#;

#[cfg(windows)]
fn run_powershell(script: &str) -> Result<String, String> {
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| format!("Impossible de lancer PowerShell : {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if !output.status.success() {
        let detail = if stderr.is_empty() { stdout } else { stderr };
        return Err(if detail.is_empty() {
            "PowerShell a échoué.".into()
        } else {
            detail
        });
    }

    Ok(stdout)
}

#[tauri::command]
pub fn windows_license_status() -> Result<WindowsLicenseInfo, String> {
    #[cfg(not(windows))]
    {
        return Err("Disponible uniquement sur Windows.".into());
    }

    #[cfg(windows)]
    {
        let json = run_powershell(LICENSE_STATUS_SCRIPT)?;
        serde_json::from_str(&json).map_err(|e| format!("Réponse licence invalide : {e}"))
    }
}

#[tauri::command]
pub fn windows_open_activation_settings() -> Result<(), String> {
    #[cfg(not(windows))]
    {
        return Err("Disponible uniquement sur Windows.".into());
    }

    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", "ms-settings:activation"])
            .spawn()
            .map_err(|e| format!("Impossible d'ouvrir les paramètres : {e}"))?;
        Ok(())
    }
}

#[tauri::command]
pub fn windows_license_placeholder_command() -> Result<String, String> {
    #[cfg(not(windows))]
    {
        return Err("Disponible uniquement sur Windows.".into());
    }

    #[cfg(windows)]
    {
        run_powershell("Write-Output 'bonjour'")
    }
}
