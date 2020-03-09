// From https://github.com/bytecodealliance/wasmtime-demos, used under Apache 2 license.

using System;
using System.Security.Cryptography;
using Wasmtime;

namespace WasmtimeDemo
{
    class Host : IHost
    {
        private RNGCryptoServiceProvider _random = new RNGCryptoServiceProvider();

        // These are from the current WASI proposal.
        const int WASI_ERRNO_NOTSUP = 58;
        const int WASI_ERRNO_SUCCESS = 0;

        public Instance Instance { get; set; }

        #region WASI Snapshot Imports

        [Import("fd_write", Module = "wasi_snapshot_preview1")]
        public int WriteFile(int fd, int iovs, int iovs_len, int nwritten)
        {
            return WASI_ERRNO_NOTSUP;
        }

        [Import("random_get", Module = "wasi_snapshot_preview1")]
        public int GetRandomBytes(int buf, int buf_len)
        {
            _random.GetBytes(Instance.Externs.Memories[0].Span.Slice(buf, buf_len));
            return WASI_ERRNO_SUCCESS;
        }

        #endregion

        #region Rust Env Externs

        [Import("__log", Module="env")]
        public void Log(int address, int length)
        {
            var message = Instance.Externs.Memories[0].ReadString(address, length);
            Console.WriteLine($"--> {message}");
        }

        #endregion

    }
}
