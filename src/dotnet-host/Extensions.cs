using System.Reflection;
using System.IO;

namespace WasmtimeDemo
{
    internal static class Extensions
    {
        internal static byte[] ReadManifestResource(this Assembly assembly, string resourceName)
        {
            var source = assembly.GetManifestResourceStream(resourceName);
            var dest = new MemoryStream();
            source.CopyTo(dest);
            return dest.GetBuffer();
        }
    }
}
