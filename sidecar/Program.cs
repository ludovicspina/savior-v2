using System.Text.Json;
using System.Text.Json.Serialization;
using LibreHardwareMonitor.Hardware;

namespace Savior.Sensord;

internal static class Program
{
    private static Computer? _computer;
    private static string _mode = "basic";
    private static int _intervalMs = 1000;
    private static volatile bool _running = true;

    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    };

    public static async Task Main()
    {
        Console.OutputEncoding = System.Text.Encoding.UTF8;
        _ = Task.Run(ReadStdinLoop);

        // Start in basic mode: no LHM Computer / no Ring0 driver loaded
        while (_running)
        {
            try
            {
                var snapshot = CollectSnapshot();
                var json = JsonSerializer.Serialize(snapshot, JsonOptions);
                Console.WriteLine(json);
                await Console.Out.FlushAsync();
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"snapshot error: {ex.Message}");
            }

            await Task.Delay(_intervalMs);
        }
    }

    private static async Task ReadStdinLoop()
    {
        while (_running)
        {
            var line = await Console.In.ReadLineAsync();
            if (line is null)
            {
                await Task.Delay(100);
                continue;
            }

            var parts = line.Trim().Split(' ', 2, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length == 0) continue;

            switch (parts[0].ToLowerInvariant())
            {
                case "enable-deep":
                    EnableDeepMode();
                    break;
                case "disable-deep":
                    DisableDeepMode();
                    break;
                case "set-interval" when parts.Length == 2 && int.TryParse(parts[1], out var ms):
                    _intervalMs = Math.Max(250, ms);
                    break;
                case "quit":
                    _running = false;
                    DisableDeepMode();
                    break;
            }
        }
    }

    private static void EnableDeepMode()
    {
        if (_computer is not null)
        {
            _mode = "deep";
            return;
        }

        try
        {
            _computer = new Computer
            {
                IsCpuEnabled = true,
                IsGpuEnabled = true,
                IsMemoryEnabled = true,
                IsMotherboardEnabled = true,
                IsControllerEnabled = true,
                IsStorageEnabled = true,
            };
            _computer.Open();
            _mode = "deep";
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"enable-deep failed: {ex.Message}");
            _computer?.Close();
            _computer = null;
            _mode = "basic";
        }
    }

    private static void DisableDeepMode()
    {
        if (_computer is not null)
        {
            _computer.Close();
            _computer = null;
        }
        _mode = "basic";
    }

    private static SensorSnapshot CollectSnapshot()
    {
        if (_computer is null)
        {
            return CollectBasicSnapshot();
        }

        _computer.Accept(new UpdateVisitor());

        var cpuName = "CPU";
        double cpuLoad = 0;
        double? cpuTemp = null;
        double? cpuClock = null;

        var gpus = new List<GpuSnapshot>();
        var disks = new List<DiskSnapshot>();

        foreach (var hardware in _computer.Hardware)
        {
            switch (hardware.HardwareType)
            {
                case HardwareType.Cpu:
                    cpuName = hardware.Name;
                    foreach (var sensor in hardware.Sensors)
                    {
                        if (sensor.Value is null) continue;
                        if (sensor.SensorType == SensorType.Load && sensor.Name.Contains("Total", StringComparison.OrdinalIgnoreCase))
                            cpuLoad = sensor.Value.Value;
                        if (sensor.SensorType == SensorType.Temperature && cpuTemp is null)
                            cpuTemp = sensor.Value.Value;
                        if (sensor.SensorType == SensorType.Clock && sensor.Name.Contains("Core", StringComparison.OrdinalIgnoreCase) && cpuClock is null)
                            cpuClock = sensor.Value.Value;
                    }
                    break;

                case HardwareType.GpuNvidia or HardwareType.GpuAmd or HardwareType.GpuIntel:
                    double? gpuLoad = null;
                    double? gpuTemp = null;
                    double? gpuMem = null;
                    foreach (var sensor in hardware.Sensors)
                    {
                        if (sensor.Value is null) continue;
                        if (sensor.SensorType == SensorType.Load && gpuLoad is null)
                            gpuLoad = sensor.Value.Value;
                        if (sensor.SensorType == SensorType.Temperature && gpuTemp is null)
                            gpuTemp = sensor.Value.Value;
                        if (sensor.SensorType == SensorType.SmallData && sensor.Name.Contains("Memory Used", StringComparison.OrdinalIgnoreCase))
                            gpuMem = sensor.Value.Value;
                    }
                    gpus.Add(new GpuSnapshot(hardware.Name, gpuLoad, gpuTemp, gpuMem is null ? null : (ulong?)gpuMem));
                    break;

                case HardwareType.Storage:
                    double? diskTemp = null;
                    double? health = null;
                    foreach (var sensor in hardware.Sensors)
                    {
                        if (sensor.Value is null) continue;
                        if (sensor.SensorType == SensorType.Temperature && diskTemp is null)
                            diskTemp = sensor.Value.Value;
                        if (sensor.SensorType == SensorType.Level && health is null)
                            health = sensor.Value.Value;
                    }
                    disks.Add(new DiskSnapshot(hardware.Name, diskTemp, health, 0, 0));
                    break;
            }
        }

        var (usedMb, totalMb) = GetMemoryMb();

        return new SensorSnapshot(
            DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            _mode,
            new CpuSnapshot(cpuName, cpuLoad, cpuTemp, cpuClock),
            new RamSnapshot(usedMb, totalMb),
            gpus,
            disks
        );
    }

    private static SensorSnapshot CollectBasicSnapshot()
    {
        var cpuLoad = 0.0;
        try
        {
            using var cpuCounter = new System.Diagnostics.PerformanceCounter("Processor", "% Processor Time", "_Total");
            cpuCounter.NextValue();
            Thread.Sleep(100);
            cpuLoad = cpuCounter.NextValue();
        }
        catch
        {
            // Performance counters may be unavailable
        }

        var (usedMb, totalMb) = GetMemoryMb();

        var disks = new List<DiskSnapshot>();
        foreach (var drive in DriveInfo.GetDrives().Where(d => d.IsReady))
        {
            var totalGb = (ulong)(drive.TotalSize / 1_073_741_824);
            var usedGb = (ulong)((drive.TotalSize - drive.AvailableFreeSpace) / 1_073_741_824);
            disks.Add(new DiskSnapshot(drive.Name, null, null, usedGb, totalGb));
        }

        return new SensorSnapshot(
            DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            _mode,
            new CpuSnapshot(Environment.GetEnvironmentVariable("PROCESSOR_IDENTIFIER") ?? "CPU", cpuLoad, null, null),
            new RamSnapshot(usedMb, totalMb),
            [],
            disks
        );
    }

    private static (ulong UsedMb, ulong TotalMb) GetMemoryMb()
    {
        try
        {
            var memStatus = new MEMORYSTATUSEX();
            if (GlobalMemoryStatusEx(memStatus))
            {
                var total = memStatus.ullTotalPhys / 1_048_576;
                var used = (memStatus.ullTotalPhys - memStatus.ullAvailPhys) / 1_048_576;
                return (used, total);
            }
        }
        catch
        {
            // fallback below
        }

        return (0, 0);
    }

    [System.Runtime.InteropServices.DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool GlobalMemoryStatusEx([System.Runtime.InteropServices.Out] MEMORYSTATUSEX lpBuffer);

    [System.Runtime.InteropServices.StructLayout(System.Runtime.InteropServices.LayoutKind.Sequential)]
    private class MEMORYSTATUSEX
    {
        public uint dwLength = (uint)System.Runtime.InteropServices.Marshal.SizeOf(typeof(MEMORYSTATUSEX));
        public uint dwMemoryLoad;
        public ulong ullTotalPhys;
        public ulong ullAvailPhys;
        public ulong ullTotalPageFile;
        public ulong ullAvailPageFile;
        public ulong ullTotalVirtual;
        public ulong ullAvailVirtual;
        public ulong ullAvailExtendedVirtual;
    }

    private sealed class UpdateVisitor : IVisitor
    {
        public void VisitComputer(IComputer computer) => computer.Traverse(this);
        public void VisitHardware(IHardware hardware)
        {
            hardware.Update();
            foreach (var sub in hardware.SubHardware)
                sub.Accept(this);
        }
        public void VisitSensor(ISensor sensor) { }
        public void VisitParameter(IParameter parameter) { }
    }
}

internal record SensorSnapshot(
    long Ts,
    string Mode,
    CpuSnapshot Cpu,
    RamSnapshot Ram,
    List<GpuSnapshot> Gpus,
    List<DiskSnapshot> Disks
);

internal record CpuSnapshot(string Name, double LoadPct, double? TempC, double? ClockMhz);
internal record RamSnapshot(ulong UsedMb, ulong TotalMb);
internal record GpuSnapshot(string Name, double? LoadPct, double? TempC, ulong? MemUsedMb);
internal record DiskSnapshot(string Name, double? TempC, double? HealthPct, ulong UsedGb, ulong TotalGb);
