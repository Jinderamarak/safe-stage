using Silk.NET.Vulkan;
using Semaphore = Silk.NET.Vulkan.Semaphore;

namespace ServiceApp.Vulkan.Render;

internal class VulkanCommandBufferPool : IDisposable
{
    private readonly Vk _api;
    private readonly CommandPool _commandPool;
    private readonly Device _device;
    private readonly object _lock = new();
    private readonly Queue _queue;

    private readonly List<VulkanCommandBuffer> _usedCommandBuffers = new();

    public unsafe VulkanCommandBufferPool(Vk api, Device device, Queue queue, uint queueFamilyIndex)
    {
        _api = api;
        _device = device;
        _queue = queue;

        var commandPoolCreateInfo = new CommandPoolCreateInfo
        {
            SType = StructureType.CommandPoolCreateInfo,
            Flags = CommandPoolCreateFlags.ResetCommandBufferBit,
            QueueFamilyIndex = queueFamilyIndex
        };

        _api.CreateCommandPool(_device, commandPoolCreateInfo, null, out _commandPool)
            .ThrowOnError();
    }

    public unsafe void Dispose()
    {
        lock (_lock)
        {
            FreeUsedCommandBuffers();
            _api.DestroyCommandPool(_device, _commandPool, null);
        }
    }

    private CommandBuffer AllocateCommandBuffer()
    {
        var commandBufferAllocateInfo = new CommandBufferAllocateInfo
        {
            SType = StructureType.CommandBufferAllocateInfo,
            CommandPool = _commandPool,
            CommandBufferCount = 1,
            Level = CommandBufferLevel.Primary
        };

        lock (_lock)
        {
            _api.AllocateCommandBuffers(_device, commandBufferAllocateInfo, out var commandBuffer);

            return commandBuffer;
        }
    }

    public VulkanCommandBuffer CreateCommandBuffer()
    {
        return new VulkanCommandBuffer(_api, _device, _queue, this);
    }

    public void FreeUsedCommandBuffers()
    {
        lock (_lock)
        {
            foreach (var usedCommandBuffer in _usedCommandBuffers) usedCommandBuffer.Dispose();

            _usedCommandBuffers.Clear();
        }
    }

    private void DisposeCommandBuffer(VulkanCommandBuffer commandBuffer)
    {
        lock (_lock)
        {
            _usedCommandBuffers.Add(commandBuffer);
        }
    }

    internal class VulkanCommandBuffer : IDisposable
    {
        private readonly Vk _api;
        private readonly VulkanCommandBufferPool _commandBufferPool;
        private readonly Device _device;
        private readonly Fence _fence;
        private readonly Queue _queue;
        private bool _hasEnded;
        private bool _hasStarted;

        internal unsafe VulkanCommandBuffer(Vk api, Device device, Queue queue,
            VulkanCommandBufferPool commandBufferPool)
        {
            _api = api;
            _device = device;
            _queue = queue;
            _commandBufferPool = commandBufferPool;

            InternalHandle = _commandBufferPool.AllocateCommandBuffer();

            var fenceCreateInfo = new FenceCreateInfo
            {
                SType = StructureType.FenceCreateInfo,
                Flags = FenceCreateFlags.SignaledBit
            };

            api.CreateFence(device, fenceCreateInfo, null, out _fence);
        }

        public IntPtr Handle => InternalHandle.Handle;

        internal CommandBuffer InternalHandle { get; }

        public unsafe void Dispose()
        {
            _api.WaitForFences(_device, 1, _fence, true, ulong.MaxValue);
            lock (_commandBufferPool._lock)
            {
                _api.FreeCommandBuffers(_device, _commandBufferPool._commandPool, 1, InternalHandle);
            }

            _api.DestroyFence(_device, _fence, null);
        }

        public void BeginRecording()
        {
            if (!_hasStarted)
            {
                _hasStarted = true;

                var beginInfo = new CommandBufferBeginInfo
                {
                    SType = StructureType.CommandBufferBeginInfo,
                    Flags = CommandBufferUsageFlags.OneTimeSubmitBit
                };

                _api.BeginCommandBuffer(InternalHandle, beginInfo);
            }
        }

        public void EndRecording()
        {
            if (_hasStarted && !_hasEnded)
            {
                _hasEnded = true;

                _api.EndCommandBuffer(InternalHandle);
            }
        }

        public void Submit()
        {
            Submit(null, null, null, _fence);
        }

        public unsafe void Submit(
            ReadOnlySpan<Semaphore> waitSemaphores,
            ReadOnlySpan<PipelineStageFlags> waitDstStageMask = default,
            ReadOnlySpan<Semaphore> signalSemaphores = default,
            Fence? fence = null,
            KeyedMutexSubmitInfo? keyedMutex = null)
        {
            EndRecording();

            if (!fence.HasValue)
                fence = _fence;


            ulong acquireKey = keyedMutex?.AcquireKey ?? 0, releaseKey = keyedMutex?.ReleaseKey ?? 0;
            var devMem = keyedMutex?.DeviceMemory ?? default;
            var timeout = uint.MaxValue;
            Win32KeyedMutexAcquireReleaseInfoKHR mutex = default;
            if (keyedMutex != null)
                mutex = new Win32KeyedMutexAcquireReleaseInfoKHR
                {
                    SType = StructureType.Win32KeyedMutexAcquireReleaseInfoKhr,
                    AcquireCount = keyedMutex.AcquireKey.HasValue ? 1u : 0u,
                    ReleaseCount = keyedMutex.ReleaseKey.HasValue ? 1u : 0u,
                    PAcquireKeys = &acquireKey,
                    PReleaseKeys = &releaseKey,
                    PAcquireSyncs = &devMem,
                    PReleaseSyncs = &devMem,
                    PAcquireTimeouts = &timeout
                };

            fixed (Semaphore* pWaitSemaphores = waitSemaphores, pSignalSemaphores = signalSemaphores)
            {
                fixed (PipelineStageFlags* pWaitDstStageMask = waitDstStageMask)
                {
                    var commandBuffer = InternalHandle;
                    var submitInfo = new SubmitInfo
                    {
                        PNext = keyedMutex != null ? &mutex : null,
                        SType = StructureType.SubmitInfo,
                        WaitSemaphoreCount = waitSemaphores != null ? (uint)waitSemaphores.Length : 0,
                        PWaitSemaphores = pWaitSemaphores,
                        PWaitDstStageMask = pWaitDstStageMask,
                        CommandBufferCount = 1,
                        PCommandBuffers = &commandBuffer,
                        SignalSemaphoreCount = signalSemaphores != null ? (uint)signalSemaphores.Length : 0,
                        PSignalSemaphores = pSignalSemaphores
                    };

                    _api.ResetFences(_device, 1, fence.Value);

                    _api.QueueSubmit(_queue, 1, submitInfo, fence.Value);
                }
            }

            _commandBufferPool.DisposeCommandBuffer(this);
        }

        internal class KeyedMutexSubmitInfo
        {
            public ulong? AcquireKey { get; set; }
            public ulong? ReleaseKey { get; set; }
            public DeviceMemory DeviceMemory { get; set; }
        }
    }
}