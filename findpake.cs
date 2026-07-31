using System;
using System.Runtime.InteropServices;
using System.Text;
public class FindPake {
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);
    [DllImport("user32.dll")] static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint lpdwProcessId);
    delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    public struct RECT { public int Left, Top, Right, Bottom; }
    public static void Main(string[] args) {
        EnumWindows((hWnd, lParam) => {
            if (IsWindowVisible(hWnd)) {
                StringBuilder sb = new StringBuilder(256);
                GetWindowText(hWnd, sb, 256);
                RECT rc;
                GetWindowRect(hWnd, out rc);
                int w = rc.Right - rc.Left;
                int h = rc.Bottom - rc.Top;
                uint pid;
                GetWindowThreadProcessId(hWnd, out pid);
                string title = sb.ToString();
                if (title.ToLower().Contains("pake") || (w > 500 && h > 300)) {
                    Console.WriteLine("PID=" + pid + " HWND=" + hWnd + " Title=\"" + title + "\" Size=" + w + "x" + h);
                }
            }
            return true;
        }, IntPtr.Zero);
    }
}
