using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Diagnostics;
public class EnumWin {
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);
    [DllImport("user32.dll")] static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint lpdwProcessId);
    delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    public struct RECT { public int Left, Top, Right, Bottom; }
    static uint targetPid;
    public static void Main(string[] args) {
        targetPid = uint.Parse(args[0]);
        EnumWindows((hWnd, lParam) => {
            uint pid;
            GetWindowThreadProcessId(hWnd, out pid);
            if (pid == targetPid && IsWindowVisible(hWnd)) {
                StringBuilder sb = new StringBuilder(256);
                GetWindowText(hWnd, sb, 256);
                RECT rc;
                GetWindowRect(hWnd, out rc);
                Console.WriteLine("HWND=" + hWnd + " Title=\"" + sb.ToString() + "\" Size=" + (rc.Right-rc.Left) + "x" + (rc.Bottom-rc.Top));
            }
            return true;
        }, IntPtr.Zero);
    }
}
