<template>
  <form @submit.prevent="updateConfig" :disabled="isSubmitting">
    <div class="form-row">
      <label for="name">Name:</label>
      <input
        v-model="formConfig.name"
        type="text"
        id="name"
        required
        placeholder="请输入配置名称，例如 XGM_MDS24"
        pattern="^[A-Za-z0-9_]+$"
        title="仅支持字母、数字和下划线"
      />
      <small class="help-text">合法名称：仅支持字母、数字和下划线</small>
    </div>
    <div class="form-row">
      <label for="addr">Addr:</label>
      <input
        v-model="formConfig.addr"
        type="text"
        id="addr"
        required
        placeholder="请输入合法的 IP 地址，如 192.168.0.1"
        pattern="^((25[0-5]|2[0-4]\d|[01]?\d?\d)\.){3}(25[0-5]|2[0-4]\d|[01]?\d?\d)$"
        title="请输入合法的 IP 地址，格式如 192.168.0.1"
      />
      <small class="help-text">合法地址：IPv4，如 192.168.0.1</small>
    </div>
    <div class="form-row">
      <label for="port">Port:</label>
      <input
        v-model="formConfig.port"
        type="number"
        id="port"
        required
        placeholder="请输入端口号，40000-65535"
        min="40000"
        max="65535"
      />
      <small class="help-text">端口范围：40000 - 65535</small>
    </div>
    <div class="form-actions">
      <button type="submit" class="submit-button" :disabled="isSubmitting">更新</button>
      <button type="button" class="cancel-button" @click="close" :disabled="isSubmitting">取消</button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';


interface Config {
  name: string;
  addr: string;
  port: number;
}


// 配置表单
const formConfig = reactive({
  name: '',
  addr: '',
  port: 40000,
});

// 接收父组件传递的设备信息
const props = defineProps<{
  device: { sn: string; name: string; mac: string; ip: string; status: string; timestamp: string } | null;
}>();

// 父组件触发的事件
const emit = defineEmits(['update', 'close']);

// 加载和更新状态
let isSubmitting = false;


const fetchConfigData = async () => {
  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.name || !props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置');
    alert('设备信息不完整，无法获取配置');
    return;
  }

  try {
    // 调用后端接口获取设备配置
    const config = await invoke<Config>('get_device_config', { 
      deviceName: props.device.name,
      deviceIp: props.device.ip,
    });

    // 检查返回的配置是否有效
    if (config) {
      // 更新表单配置
      formConfig.name = config.name || '';
      formConfig.addr = config.addr || '';
      formConfig.port = config.port || 40000;  // 使用默认端口
    } else {
      alert('没有找到该设备的配置');
    }
  } catch (err: any) {
    // 捕获和处理错误
    console.error('获取配置数据失败:', err);
    const errorMessage = err?.response?.data?.error || err.message || '未知错误';
    alert(`获取配置数据失败: ${errorMessage}`);
  }
};


watch(() => props.device, async (newDevice) => {
  if (newDevice?.name && newDevice?.ip) {
    await fetchConfigData();
  }
}, { immediate: true }); // immediate: true 表示组件加载时就会调用一次


// 更新配置
const updateConfig = async () => {
  if (isSubmitting) return; // 防止重复提交
  isSubmitting = true;

  try {
    await invoke('update_device_config', {
      deviceName: formConfig.name,
      deviceIp: props.device?.ip,  
      configData: formConfig,
    });
    alert('配置更新成功');
    emit('update');
    emit('close');
  } catch (err: any) {
    console.error('更新配置失败:', err);
    alert("更新失败：" + (err.response?.data?.error || err.message));
  } finally {
    isSubmitting = false;  // 恢复按钮状态
  }
};

// onMounted(() => {
//   fetchConfigData();
// });

// 取消操作
const close = () => {
  emit('close');
};
</script>

<style scoped>
.form-row {
  margin-bottom: 12px;
}

.form-row label {
  display: block;
  margin-bottom: 4px;
  font-weight: bold;
}

.form-row input {
  width: 100%;
  padding: 8px;
  font-size: 16px;
  box-sizing: border-box;
  border-radius: 4px;
  border: 1px solid #ccc;
  transition: border 0.3s ease;
}

.form-row input:focus {
  border-color: #42b983;
  outline: none;
}

.help-text {
  display: block;
  margin-top: 4px;
  color: #666;
  font-size: 12px;
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
