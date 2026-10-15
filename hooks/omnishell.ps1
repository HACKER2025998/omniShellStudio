# Chargez ce fichier dans votre profil : . 'C:\chemin\omnishell.ps1'.
# Conservez la fonction d'invite installée par votre thème ou terminal.
if (-not $global:OmniShellOriginalPrompt) { $global:OmniShellOriginalPrompt = (Get-Item Function:\prompt).ScriptBlock }
# La capture intervient à l'invite et ne réexécute jamais la commande.
function global:prompt {
    # Get-History fournit la commande complète de la dernière entrée interactive.
    $lastEntry = Get-History -Count 1
    # Les rafraîchissements d'invite ne doivent pas produire de doublon.
    if ($lastEntry -and $global:OmniShellLastId -ne $lastEntry.Id) {
        $global:OmniShellLastId = $lastEntry.Id
        try {
            # ProcessStartInfo transmet les données via stdin, jamais via une chaîne shell.
            $info = [System.Diagnostics.ProcessStartInfo]::new('omnishell', 'capture-hook')
            $info.UseShellExecute = $false
            $info.RedirectStandardInput = $true
            $info.RedirectStandardOutput = $true
            $info.RedirectStandardError = $true
            $info.CreateNoWindow = $true
            $info.EnvironmentVariables['OMNISHELL_SHELL'] = 'pwsh'
            # Les anciennes versions PowerShell ne fournissent pas un exit_code fiable pour les cmdlets.
            $process = [System.Diagnostics.Process]::Start($info)
            $process.StandardInput.Write($lastEntry.CommandLine)
            $process.StandardInput.Close()
            # Disposer la poignée n'arrête pas le processus enfant asynchrone.
            $process.Dispose()
        } catch { # Une absence du binaire ne doit jamais casser l'invite.
        }
    }
    # Le thème d'invite d'origine reste propriétaire de son affichage.
    & $global:OmniShellOriginalPrompt
}
