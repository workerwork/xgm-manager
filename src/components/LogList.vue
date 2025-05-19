<template>
  <div class="log-wrapper">
    <h2>quark 日志</h2>
    <div class="log-box">
      <pre>{{ logOutput }}</pre>
    </div>
    <div class="button-container">
      <button @click="fetchLogs">刷新日志</button>
      <button @click="clearLogs">清除日志</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const logOutput = ref('加载中...')

// 接收父组件传递的设备信息
const props = defineProps<{
  device: { sn: string; name: string; mac: string; ip: string; status: string; timestamp: string } | null;
}>()

async function fetchLogs() {
  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置')
    alert('设备信息不完整，无法获取配置')
    return
  }

  try {
    const logs: string[] = await invoke('get_log_list', {
      deviceIp: props.device.ip,
    }) // Rust 后端暴露的命令名
    logOutput.value = logs.join('\n') || '无日志'
  } catch (error) {
    logOutput.value = `❌ 获取日志失败: ${error}`
  }
}

async function clearLogs() {
  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法清除日志')
    alert('设备信息不完整，无法清除日志')
    return
  }

  try {
    await invoke('clear_log_list', { deviceIp: props.device.ip }) // 调用后端清除日志的命令
    logOutput.value = '日志已清除'
  } catch (error) {
    logOutput.value = `❌ 清除日志失败: ${error}`
  }
}

watch(() => props.device, async (newDevice) => {
  if (newDevice?.ip) {
    await fetchLogs();
  }
}, { immediate: true }); // immediate: true 表示组件加载时就会调用一次

</script>

<style scoped>
.log-wrapper {
  padding: 1rem;
  font-family: 'Courier New', Courier, monospace;
}

.log-box {
  height: 600px;
  overflow-y: auto;
  border: 1px solid #ccc;
  background: #f9f9f9;
  padding: 1rem;
  white-space: pre-wrap;
  margin-bottom: 1rem;
}

.button-container {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

button {
  padding: 8px 16px;
  background-color: #007bff;
  color: white;
  border: none;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #0056b3;
}
</style>
