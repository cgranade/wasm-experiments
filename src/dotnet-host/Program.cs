// From https://github.com/bytecodealliance/wasmtime-demos, used under Apache 2 license.

using System;
using System.Linq;
using Wasmtime;

namespace WasmtimeDemo
{
    public static class Program
    {
        public static void Main()
        {
            // TODO: convert to use EngineBuilder.
            using var engine = new Engine();
            using var store = engine.CreateStore();

            using var module = store.CreateModule("wasmlib.wasm", typeof(Program).Assembly.ReadManifestResource("Host.wasmlib.wasm"));
            using var instance = module.Instantiate(new Host());

            var memory = instance.Externs.Memories.SingleOrDefault() ??
                throw new InvalidOperationException("Module must export a memory.");

            var allocator = new Allocator(memory, instance.Externs.Functions);

            var resourceId = instance.Create();


            (var inputAddress, var inputLength) = allocator.AllocateString("Hello, Rust!");

            try
            {
                object[] results = (instance as dynamic).render(inputAddress, inputLength);

                var outputAddress = (int)results[0];
                var outputLength = (int)results[1];

                try
                {
                    Console.WriteLine(memory.ReadString(outputAddress, outputLength));
                }
                finally
                {
                    allocator.Free(outputAddress, outputLength);
                }
            }
            finally
            {
                allocator.Free(inputAddress, inputLength);
            }
        }

        private static uint Create(this Instance instance)
        {
            return (uint) (instance as dynamic).@new();
        }

        public static void X(this Instance instance, int resourceId)
        {
            (instance as dynamic).x(resourceId);
        }

    }
}
