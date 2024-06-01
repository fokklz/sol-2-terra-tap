<#
.SYNOPSIS
This script copies subdirectories to the Arduino libraries directory.

.DESCRIPTION
The script retrieves the path of the user's documents folder and the Arduino libraries directory. 
It then loops through each subdirectory in the source directory and moves it to the Arduino libraries directory.
If a subdirectory with the same name already exists in the destination directory, it is removed before copying the new one.

.PARAMETER None

.NOTES
Author: Fokko Vos
Version: 1.0
#>

$documentsPath = [Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)
$arduinoLibrariesDirectory = Join-Path -Path $documentsPath -ChildPath "Arduino\libraries"
$sourceDirectory = $PSScriptRoot
$subDirectories = Get-ChildItem -Path $sourceDirectory -Directory

# Loop through each subdirectory and move it to the Arduino libraries directory
foreach ($subDir in $subDirectories) {
    $destPath = Join-Path -Path $arduinoLibrariesDirectory -ChildPath $subDir.Name
    
    # Check if the destination directory already exists
    if (Test-Path -Path $destPath) {
        # Remove existing directory
        Remove-Item -Path $destPath -Recurse -Force
    }

    # Copy the directory
    Copy-Item -Path $subDir.FullName -Destination $arduinoLibrariesDirectory -Recurse -Force
    Write-Host "Copied $($subDir.Name) to the Arduino libraries folder."
}

Write-Host "All folders have been copied successfully."
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")