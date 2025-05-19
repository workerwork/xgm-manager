<template>
  <div class="config-container">
    <form @submit.prevent="updateNetworkConfig">
      <div class="form-group">
        <label for="ip">IP 地址：</label>
        <input
          v-model="networkConfig.ip"
          type="text"
          id="ip"
          required
          placeholder="请输入合法的 IP 地址，如 192.168.0.2"
          pattern="^((25[0-5]|2[0-4]\d|[01]?\d?\d)\.){3}(25[0-5]|2[0-4]\d|[01]?\d?\d)$"
          title="请输入合法的 IP 地址，格式如 192.168.0.2"
          class="form-input"
        />
        <small class="help-text">地址：IPv4，如 192.168.0.2</small>
      </div>
      <div class="form-group">
        <label for="netmask">子网掩码：</label>
        <input
          v-model="networkConfig.netmask"
          type="text"
          id="netmask"
          required
          placeholder="请输入合法的子网掩码，如 255.255.255.0"
          pattern="^((255\.){3}(255|254|252|248|240|224|192|128|0+))$"
          title="请输入合法的子网掩码地址，格式如 255.255.255.0"
          class="form-input"
        />
        <small class="help-text">子网掩码：IPv4，如 255.255.255.0</small>
      </div>
      <!-- <div class="form-group">
        <label for="gateway">网关：</label>
        <input
          v-model="networkConfig.gateway"
          type="text"
          id="gateway"
          placeholder="请输入合法的 IP 地址，如 192.168.0.1"
          pattern="^((25[0-5]|2[0-4]\d|[01]?\d?\d)\.){3}(25[0-5]|2[0-4]\d|[01]?\d?\d)$"
          class="form-input"
        />
        <small class="help-text">网关：IPv4，如 192.168.0.1</small>
      </div> -->
      <div class="form-actions">
        <button type="submit" class="submit-button">保存</button>
        <button type="button" class="cancel-button" @click="close">取消</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 接收父组件传递的设备信息
const props = defineProps<{
  device: { sn: string; name: string; mac: string; ip: string; status: string; timestamp: string } | null;
}>();

const emit = defineEmits(['close', 'updateNetwork']);

const networkConfig = reactive({
  ip: '',
  netmask: '',
});

const updateNetworkConfig = async () => {
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置');
    alert('设备信息不完整，无法获取配置');
    return;
  }

  try {
    // 调用后端的 Rust 方法来更新网卡配置
    await invoke('update_network_config', {
      deviceIp: props.device.ip,
      ip: networkConfig.ip,
      netmask: networkConfig.netmask,
    });
    
    // 输出成功的信息并通知父组件
    emit('updateNetwork');  // 通知父组件
    emit('close');  // 关闭模态框

  } catch (error) {
    console.error('网络配置更新失败:', error);
    alert('更新失败，请重试');
  }
};

const close = () => {
  emit('close');
};
</script>

<style scoped>
.config-container {
  width: 100%;
  max-width: 500px;
  margin: 0 auto;
}

.form-group {
  margin-bottom: 12px;
}

.form-group label {
  display: block;
  font-weight: bold;
  margin-bottom: 6px;
}

.form-input {
  width: 100%;
  padding: 8px;
  font-size: 16px;
  border-radius: 4px;
  border: 1px solid #ccc;
  box-sizing: border-box;
}

.form-actions {
  text-align: right;
  margin-top: 16px;
}

.submit-button,
.cancel-button {
  padding: 10px 20px;
  font-size: 16px;
  font-weight: bold;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s, transform 0.3s;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  margin-right: 12px;
}

.submit-button {
  background-color: #4CAF50;
  color: white;
}

.cancel-button {
  background-color: #f44336;
  color: white;
}

.submit-button:hover {
  background-color: #45a049;
  transform: translateY(-2px);
}

.cancel-button:hover {
  background-color: #e53935;
  transform: translateY(-2px);
}

.submit-button:focus,
.cancel-button:focus {
  outline: none;
  box-shadow: 0 0 0 4px rgba(0, 123, 255, 0.3);
}
</style>
