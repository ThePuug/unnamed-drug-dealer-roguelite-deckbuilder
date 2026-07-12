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
#   SOW-030 ledger: LEDGER tab (575, 40), CLOSE (1843, 79); panels at
#     x centers ~304 (roster) / ~884 (board); first dossier ~(380, 290)
#   SOW-029 city map: CITY MAP tab (415, 40), CLOSE (1843, 79),
#     node i in shop_locations.ron order: center x = 450|960|1470,
#     node action button (UNLOCK / SEND HERE): (450|960|1470, 887),
#     first dealer chip on a node: x = 450|960|1470, y = 533 + 28 per
#     clientele line beyond the first (chip row sits below the CLIENTELE
#     list, so nodes with 2 personas - e.g. the Block - are at y ~561;
#     verify against a fresh screenshot before scripting chip clicks)
#   (zone unlocks moved to the map - the shop selector row lists unlocked
#   areas only)

param(
  [string]$Scenario = "fresh",
  [int]$Hands = 2,
  [int]$SelectDealer = -1,
  [switch]$Hire,
  [string]$BuyArea = "",   # SOW-024: buy this area in the shop before the run (e.g. the_block)
  [string]$OutDir = "$env:TEMP\ddd-playtest",
  # SOW-026 pacing runs: share one save across multiple sessions
  [string]$SaveDir = "",   # override the save location (default: OutDir\save)
  [switch]$NoForge         # continue on the existing save instead of forging
)

$ErrorActionPreference = "Stop"
$repo = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$drv = Join-Path $PSScriptRoot "game-drive.ps1"
New-Item -ItemType Directory -Force $OutDir | Out-Null
$log = Join-Path $OutDir "game.log"
$saveDir = if ($SaveDir -ne "") { $SaveDir } else { Join-Path $OutDir "save" }
$env:DDD_SAVE_DIR = $saveDir

Set-Location $repo

# 1. Forge the scenario into the isolated save dir (skipped with -NoForge -
# SOW-026 pacing sessions continue on the previous session's save)
# (via cmd so cargo's stderr warnings can't become PS 5.1 NativeCommandErrors)
if (-not $NoForge) {
  & $drv -Action reset | Out-Null
  $forgeLog = Join-Path $OutDir "forge.log"
  cmd /c "cargo run --quiet -- forge $Scenario --dir `"$saveDir`" > `"$forgeLog`" 2>&1"
  if ($LASTEXITCODE -ne 0) { Get-Content $forgeLog -Tail 5; throw "save-forge failed for scenario '$Scenario'" }
}

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

# 3a. SOW-024/SOW-029: optional territory purchase (city map -> node UNLOCK)
if ($BuyArea -ne "") {
  # Node index follows shop_locations.ron definition order
  $nodeX = switch ($BuyArea) {
    "the_corner" { 450 } "the_strip" { 960 } "the_block" { 1470 }
    default { throw "unknown area '$BuyArea'" }
  }
  & $drv -Action click -X 415 -Y 40 | Out-Null   # CITY MAP tab
  Start-Sleep -Milliseconds 800
  & $drv -Action shot -OutFile (Join-Path $OutDir "01-map-locked.png") | Out-Null
  & $drv -Action click -X $nodeX -Y 887 | Out-Null   # node UNLOCK button
  Start-Sleep -Milliseconds 1000
  & $drv -Action shot -OutFile (Join-Path $OutDir "02-after-unlock.png") | Out-Null
  & $drv -Action click -X 1843 -Y 79 | Out-Null  # CLOSE the map
  Start-Sleep -Milliseconds 500
}

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
    # Only END RUN / NEW EMPIRE is available. A kingpin GAME OVER panel is
    # taller (the fallen-empires board) so the button sits lower - try the
    # standard position, poll the log for the DeckBuilding return, then the
    # lower position (SOW-023 papercut fix).
    function DB-Count { @(Select-String -Path $log -Pattern "DeckBuilder").Count }
    $db = DB-Count
    & $drv -Action click -X 1056 -Y 694 | Out-Null
    foreach ($i in 1..6) { if ((DB-Count) -gt $db) { break }; Start-Sleep -Milliseconds 500 }
    if ((DB-Count) -le $db) {
      & $drv -Action click -X 1056 -Y 732 | Out-Null   # GAME OVER board layout
      Start-Sleep 2
    }
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
$summary += (Select-String -Path $log -Pattern "transferred|jailed|GAME OVER|hired|released|Run area|Unlocked area" | ForEach-Object { $_.Line }) | Select-Object -Last 10
$summaryPath = Join-Path $OutDir "summary.txt"
$summary | Set-Content $summaryPath -Encoding utf8
Write-Output "--- summary ---"
Get-Content $summaryPath

# 8. Cleanup
Stop-Process -Name "drug-dealer-deckbuilder" -Force -Confirm:$false -ErrorAction SilentlyContinue
Remove-Item Env:\DDD_SAVE_DIR -ErrorAction SilentlyContinue
