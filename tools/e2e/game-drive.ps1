# game-drive.ps1 - minimal e2e driver for the running game window (SOW-022)
#
# Actions (all coordinates are 1920x1080 DESIGN-space; the script converts
# through the UiScale letterbox + monitor DPI automatically):
#   shot  -OutFile out.png     occlusion-proof screenshot (PrintWindow/DWM)
#   click -X 1798 -Y 987       real mouse click (verifies game is foreground first)
#   hover -X 1480 -Y 170       move cursor only (drives Interaction::Hovered)
#
# Prereq: the game is running (cargo run). Example session:
#   ./tools/e2e/game-drive.ps1 -Action shot  -OutFile db.png
#   ./tools/e2e/game-drive.ps1 -Action click -X 1798 -Y 987   # START RUN
#   ./tools/e2e/game-drive.ps1 -Action click -X 1565 -Y 930   # PASS
#   ./tools/e2e/game-drive.ps1 -Action click -X 960  -Y 950   # middle hand card
#
# Notes:
# - Clicks are REAL input: the script brings the game to the foreground and
#   aborts if that fails, so strays can't hit other apps. Posted WM_* mouse
#   messages do NOT drive winit/bevy picking - don't bother.
# - Capture uses PrintWindow(PW_RENDERFULLCONTENT), which composites the GPU
#   swapchain even when the window is occluded, but NOT while minimized.
# - The design->screen mapping mirrors scale_ui_to_fit_system: bevy computes
#   UiScale from LOGICAL window size; this script measures PHYSICAL px, so it
#   divides by the window's DPI scale (auto-detected via GetDpiForWindow).

param(
  [string]$Action = "shot",   # shot | click | hover | reset
  [string]$OutFile = "shot.png",
  [double]$X = 0,             # position in 1920x1080 DESIGN coords
  [double]$Y = 0
)

# reset: delete the save (Reed-approved between-playtest hygiene). Honors
# DDD_SAVE_DIR like the game does; runs without the game being up.
if ($Action -eq "reset") {
  $saveDir = if ($env:DDD_SAVE_DIR) { $env:DDD_SAVE_DIR } else { Join-Path $env:LOCALAPPDATA "DrugDealerDeckbuilder" }
  foreach ($f in "save.dat", "save.dat.bak") {
    $p = Join-Path $saveDir $f
    if (Test-Path $p) { Remove-Item $p -Force -Confirm:$false }
  }
  Write-Output ("reset save at " + $saveDir)
  exit 0
}
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class Win32Drv {
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr dc, uint flags);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT wr);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out POINT p);
  [DllImport("user32.dll")] public static extern bool MoveWindow(IntPtr h, int x, int y, int w, int ht, bool repaint);
  [DllImport("user32.dll")] public static extern void mouse_event(uint flags, uint dx, uint dy, uint data, UIntPtr extra);
  [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
  public struct RECT { public int Left, Top, Right, Bottom; }
  public struct POINT { public int X, Y; }
}
'@
[Win32Drv]::SetProcessDPIAware() | Out-Null
Add-Type -AssemblyName System.Drawing

$game = Get-Process -Name "drug-dealer-deckbuilder" -ErrorAction Stop
$h = $game.MainWindowHandle
if ($h -eq [IntPtr]::Zero) { throw "no main window handle" }

$dpiRaw = [Win32Drv]::GetDpiForWindow($h)
$DPI = if ($dpiRaw -gt 0) { $dpiRaw / 96.0 } else { 1.0 }

# If the window is larger than the screen, click targets can fall off-screen
# and SetCursorPos silently clamps to the screen edge (hazard: the bottom-right
# corner is Windows' "show desktop" button). Fit the window first.
Add-Type -AssemblyName System.Windows.Forms
$screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds  # physical (DPI-aware)
$wrFit = New-Object Win32Drv+RECT
[Win32Drv]::GetWindowRect($h, [ref]$wrFit) | Out-Null
if (($wrFit.Right - $wrFit.Left) -gt $screen.Width -or ($wrFit.Bottom - $wrFit.Top) -gt $screen.Height -or $wrFit.Left -lt 0 -or $wrFit.Top -lt 0) {
  $fitW = [Math]::Min($wrFit.Right - $wrFit.Left, $screen.Width - 40)
  $fitH = [Math]::Min($wrFit.Bottom - $wrFit.Top, $screen.Height - 60)
  [Win32Drv]::MoveWindow($h, 10, 10, $fitW, $fitH, $true) | Out-Null
  Start-Sleep -Milliseconds 700
}

$r = New-Object Win32Drv+RECT
[Win32Drv]::GetClientRect($h, [ref]$r) | Out-Null
$wPhys = $r.Right - $r.Left     # physical px (this process is DPI-aware)
$htPhys = $r.Bottom - $r.Top
$wLog = $wPhys / $DPI           # what bevy's window.width() sees
$htLog = $htPhys / $DPI

# design(1920x1080) -> logical mapping mirrors scale_ui_to_fit_system
$s = [Math]::Min($wLog / 1920.0, $htLog / 1080.0)
$offx = ($wLog - 1920.0 * $s) / 2.0
$offy = ($htLog - 1080.0 * $s) / 2.0

if ($Action -eq "click" -or $Action -eq "hover") {
  $pt = New-Object Win32Drv+POINT
  $pt.X = 0; $pt.Y = 0
  [Win32Drv]::ClientToScreen($h, [ref]$pt) | Out-Null
  $sx = [int]($pt.X + ($offx + $X * $s) * $DPI)
  $sy = [int]($pt.Y + ($offy + $Y * $s) * $DPI)
  # Windows blocks SetForegroundWindow from background processes; pulsing ALT
  # (keybd_event 0x12) is the documented unlock. Retry a few times.
  for ($i = 0; $i -lt 5; $i++) {
    [Win32Drv]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero)      # ALT down
    [Win32Drv]::SetForegroundWindow($h) | Out-Null
    [Win32Drv]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)      # ALT up
    Start-Sleep -Milliseconds 300
    if ([Win32Drv]::GetForegroundWindow() -eq $h) { break }
  }
  if ([Win32Drv]::GetForegroundWindow() -ne $h) { throw "game window not foreground - aborting to avoid stray clicks" }
  $wr = New-Object Win32Drv+RECT
  [Win32Drv]::GetWindowRect($h, [ref]$wr) | Out-Null
  if ($sx -lt $wr.Left -or $sx -gt $wr.Right -or $sy -lt $wr.Top -or $sy -gt $wr.Bottom) { throw "target outside window ($sx,$sy)" }
  [Win32Drv]::SetCursorPos($sx, $sy) | Out-Null
  Start-Sleep -Milliseconds 250
  # Verify the cursor actually landed (SetCursorPos clamps at screen edges -
  # a clamped click can hit other UI, e.g. the "show desktop" corner)
  $cur = New-Object Win32Drv+POINT
  [Win32Drv]::GetCursorPos([ref]$cur) | Out-Null
  if ([Math]::Abs($cur.X - $sx) -gt 2 -or [Math]::Abs($cur.Y - $sy) -gt 2) {
    throw "cursor clamped to $($cur.X),$($cur.Y) (wanted $sx,$sy) - aborting"
  }
  if ($Action -eq "click") {
    [Win32Drv]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)  # LEFTDOWN
    Start-Sleep -Milliseconds 60
    [Win32Drv]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)  # LEFTUP
  }
  Write-Output ("$Action design " + $X + "," + $Y + " -> screen " + $sx + "," + $sy + " (fit " + [Math]::Round($s,3) + ", dpi " + $DPI + ")")
} else {
  $wr = New-Object Win32Drv+RECT
  [Win32Drv]::GetWindowRect($h, [ref]$wr) | Out-Null
  $ww = $wr.Right - $wr.Left
  $wh = $wr.Bottom - $wr.Top
  $bmp = New-Object System.Drawing.Bitmap($ww, $wh)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $dc = $g.GetHdc()
  [Win32Drv]::PrintWindow($h, $dc, 2) | Out-Null  # PW_RENDERFULLCONTENT
  $g.ReleaseHdc($dc)
  $bmp.Save($OutFile)
  $g.Dispose(); $bmp.Dispose()
  Write-Output ("saved " + $OutFile + " (window " + $ww + "x" + $wh + " phys, client " + $wPhys + "x" + $htPhys + ", fit " + [Math]::Round($s,3) + ", dpi " + $DPI + ")")
}
