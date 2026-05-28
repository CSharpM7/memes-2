$scriptPath = "..\..\scripts\build.ps1"
$arguments = @("$(Get-Location)") + $args

Invoke-Expression "& `"$scriptPath`" $arguments"