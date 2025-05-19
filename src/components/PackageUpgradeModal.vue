<template>
  <div v-if="show" class="modal-overlay" @click.self="close">
    <div class="modal-content upgrade-modal">
      <span class="close-icon" @click="close">×</span>
      <!-- <p><strong>包名：</strong>{{ pkg.name }}</p> -->
      <p>上传新版本的文件进行升级：</p>
      <!-- 选择文件按钮 -->
      <!-- <button @click="chooseFile" :disabled="!!selectedFilePath">选择文件</button> -->
      <button @click="chooseFile">选择文件</button>
      <p v-if="selectedFilePath">已选择文件：{{ selectedFilePath }}</p>
      <div class="modal-footer">
        <button class="cancel-button" @click="close">取消</button>
        <!-- 上传按钮 -->
        <button class="action-button" @click="handleUpload" :disabled="!selectedFilePath">执行升级</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';

const props = defineProps<{
  show: boolean;
  pkg: { name: string; version: string };  // 使用对象类型
  device: { sn: string; name: string; mac: string; ip: string; status: string; timestamp: string } | null;
  selectedFilePath: string | null;
}>();


const emit = defineEmits(['close', 'upload']);

// 用于保存文件路径
const selectedFilePath = ref<string | null>(null);

// 选择文件后保存文件路径
const chooseFile = async () => {
  // 每次选择文件前重置路径
  selectedFilePath.value = null; // 或者空字符串 ""

  // 如果已经选择了文件，不再弹出选择框
  if (selectedFilePath.value) {
    return;
  }

  const file = await open({
    multiple: false,
    filters: [{ name: 'Python Package', extensions: ['whl'] }]
  });

  if (typeof file === 'string') {
    selectedFilePath.value = file;  // 保存选中文件的路径
  }
};

// 执行上传
const handleUpload = async () => {
  if (!selectedFilePath.value) {
    alert("请先选择文件");
    return;
  }

  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置');
    alert('设备信息不完整，无法获取配置');
    return;
  }

  try {
    await invoke("upload_package", {
      filePath: selectedFilePath.value,
      deviceIp: props.device.ip,
    });

    alert("上传并升级成功");
    emit("upload");
    emit("close");
  } catch (err: any) {
    alert("升级失败：" + (err?.message || err));
  }
};

// 关闭模态框
const close = () => {
  selectedFilePath.value = null;  // 清空选择的文件路径
  emit('close');
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.4);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  position: relative;
  background: #fff;
  padding: 20px 30px;
  border-radius: 8px;
  width: 500px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.2);
}

.upgrade-modal {
  padding-bottom: 60px;
  position: relative;
}

.modal-footer {
  position: absolute;
  bottom: 20px;
  right: 30px;
  display: flex;
  gap: 10px;
}

.close-icon {
  position: absolute;
  top: 10px;
  right: 14px;
  font-size: 20px;
  font-weight: bold;
  color: #555;
  cursor: pointer;
}

.close-icon:hover {
  color: #000;
}

.action-button {
  padding: 4px 12px;
  border: 1px solid #007bff;
  background-color: #007bff;
  color: #fff;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.cancel-button {
  padding: 4px 12px;
  border: 1px solid #999;
  background-color: #fff;
  color: #333;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.action-button:hover {
  background-color: #0056b3;
}

.cancel-button:hover {
  background-color: #eee;
}
</style>
