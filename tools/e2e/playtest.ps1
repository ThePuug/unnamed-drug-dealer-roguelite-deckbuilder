# playtest.ps1 - closed-loop scripted playtest (SOW-023)
#
# Forges a save scenario, launches the game in an ISOLATED save dir, plays
# hands by clicking the fan, and reacts to actual outcomes by tailing the
# game's stdout log (the earlier open-loop script busted on hand 1 without
# noticing). Emits a summary of outcomes per hand.
#
# Usage:
#   ./tools/e2e/playtest.ps1 -Scenario roster -Hands 2 -OutDir out\pt
#   -SelectDealer 1   click roster row N (0-based) before START RUN
#   -Hire             click the HIRE card before START RUN
#
# Reference clicks (1920x1080 design space):
#   roster row i center: x = 30 + i*260 + 125, y = 120   (250px cards, 10px gap)
#   HIRE card (after n dealers): x = 30 + n*260 + 70, y = 120
#   START RUN (856,987 is NEW DEAL; START RUN bottom-right): (1798, 987)
#   hand fan slots: (822|960|1098, 950)   PASS: (1565, 930)
#   overlay: NEW DEAL (856, 694), GO HOME / END RUN / NEW EMPIRE (1056, 694)

param(
  [string]$Scenario = "fresh",
  [int]$Hands = 2,
  [int]$SelectDealer = -1,
  [switch]$Hire,
  [string]$OutDir = "$env:TEMP\ddd-playtest"
)

$ErrorActionPreference = "Stop"
$repo = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$drv = Join-Path $PSScriptRoot "game-drive.ps1"
New-Item -ItemType Directory -Force $OutDir | Out-Null
$log = Join-Path $OutDir "game.log"
$saveDir = Join-Path $OutDir "save"
$env:DDD_SAVE_DIR = $saveDir

Set-Location $repo

# 1. Forge the scenario into the isolated save dir
# (via cmd so cargo's stderr warnings can't become PS 5.1 NativeCommandErrors)
& $drv -Action reset | Out-Null
$forgeLog = Join-Path $OutDir "forge.log"
cmd /c "cargo run --quiet -- forge $Scenario --dir `"$saveDir`" > `"$forgeLog`" 2>&1"
if ($LASTEXITCODE -ne 0) { Get-Content $forgeLog -Tail 5; throw "save-forge failed for scenario '$Scenario'" }

# 2. Launch (stdout -> log; DDD_SAVE_DIR inherited by the child)
if (Test-Path $log) { Remove-Item $log -Force -Confirm:$false }
Start-Process -FilePath "cmd" -ArgumentList "/c", "cargo run > `"$log`" 2>&1" -WindowStyle Minimized
$up = $false
foreach ($i in 1..40) {
  Start-Sleep 2
  if (Get-Process -Name "drug-dealer-deckbuilder" -ErrorAction SilentlyContinue) { $up = $true; break }
}
if (-not $up) { Get-Content $log -Tail 10 -ErrorAction SilentlyContinue; throw "game did not start" }
Start-Sleep 5

function Outcome-Count {
  if (-not (Test-Path $log)) { return 0 }
  @(Select-String -Path $log -Pattern "Resolution outcome:" -SimpleMatch).Count
}

& $drv -Action shot -OutFile (Join-Path $OutDir "00-deckbuilder.png") | Out-Null

# 3. Optional roster actions before the run
if ($Hire) {
  # HIRE card sits after the dealer cards; count dealers from the forge scenario
  $dealerCount = switch ($Scenario) { "roster" { 3 } default { 1 } }
  $hireX = 30 + $dealerCount * 260 + 70
  & $drv -Action click -X $hireX -Y 120 | Out-Null
  Start-Sleep -Milliseconds 800
  & $drv -Action shot -OutFile (Join-Path $OutDir "01-after-hire.png") | Out-Null
}
if ($SelectDealer -ge 0) {
  $selX = 30 + $SelectDealer * 260 + 125
  & $drv -Action click -X $selX -Y 120 | Out-Null
  Start-Sleep -Milliseconds 800
  & $drv -Action shot -OutFile (Join-Path $OutDir "01-after-select.png") | Out-Null
}

# 4. START RUN
& $drv -Action click -X 1798 -Y 987 | Out-Null
Start-Sleep 2

# 5. Closed-loop hands
$outcomes = @()
$resolved = Outcome-Count
for ($hand = 1; $hand -le $Hands; $hand++) {
  # play up to 3 rounds: narc waits ~1s, we click fan slots, buyer waits ~1s
  foreach ($round in 1..3) {
    Start-Sleep -Milliseconds 2600
    foreach ($x in 822, 960, 1098) {
      try { & $drv -Action click -X $x -Y 950 | Out-Null } catch {}
      Start-Sleep -Milliseconds 400
    }
    Start-Sleep -Milliseconds 2600
    if ((Outcome-Count) -gt $resolved) { break }  # resolved early (e.g. bail out)
  }

  # wait for the resolution line to land
  foreach ($i in 1..20) {
    if ((Outcome-Count) -gt $resolved) { break }
    Start-Sleep -Milliseconds 500
  }
  if ((Outcome-Count) -le $resolved) { Write-Output "hand ${hand}: no resolution detected - aborting"; break }
  $resolved = Outcome-Count

  $line = (Select-String -Path $log -Pattern "Resolution outcome:" -SimpleMatch | Select-Object -Last 1).Line
  $outcome = if ($line -match "outcome: (\w+)") { $Matches[1] } else { "Unknown" }
  $outcomes += $outcome
  & $drv -Action shot -OutFile (Join-Path $OutDir ("hand{0}-{1}.png" -f $hand, $outcome)) | Out-Null
  Write-Output ("hand {0}: {1}" -f $hand, $outcome)

  Start-Sleep -Milliseconds 800
  if ($outcome -eq "Busted") {
    # only GO HOME (END RUN / NEW EMPIRE) is available
    & $drv -Action click -X 1056 -Y 694 | Out-Null
    Start-Sleep 2
    break
  } elseif ($hand -ge $Hands) {
    & $drv -Action click -X 1056 -Y 694 | Out-Null   # GO HOME (bank it)
    Start-Sleep 2
  } else {
    & $drv -Action click -X 856 -Y 694 | Out-Null    # NEW DEAL
    Start-Sleep 2
  }
}

# 6. Post-session roster state
Start-Sleep 2
& $drv -Action shot -OutFile (Join-Path $OutDir "99-post-session.png") | Out-Null

# 7. Summary
$summary = @()
$summary += "scenario: $Scenario"
$summary += "outcomes: " + ($outcomes -join ", ")
$summary += (Select-String -Path $log -Pattern "transferred|jailed|GAME OVER|hired|released" | ForEach-Object { $_.Line }) | Select-Object -Last 10
$summaryPath = Join-Path $OutDir "summary.txt"
$summary | Set-Content $summaryPath -Encoding utf8
Write-Output "--- summary ---"
Get-Content $summaryPath

# 8. Cleanup
Stop-Process -Name "drug-dealer-deckbuilder" -Force -Confirm:$false -ErrorAction SilentlyContinue
Remove-Item Env:\DDD_SAVE_DIR -ErrorAction SilentlyContinue
