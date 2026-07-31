using System;
using System.Runtime.InteropServices;
public class CheckWin {
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    public struct RECT { public int Left, Top, Right, Bottom; }
    public static void Main(string[] args) {
        IntPtr hWnd = new IntPtr(long.Parse(args[0]));
        RECT rc;
        GetWindowRect(hWnd, out rc);
        bool vis = IsWindowVisible(hWnd);
        Console.WriteLine("Visible: " + vis);
        Console.WriteLine("Rect: " + rc.Left + "," + rc.Top + " " + (rc.Right-rc.Left) + "x" + (rc.Bottom-rc.Top));
    }
}
