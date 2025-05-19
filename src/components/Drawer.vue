<template>
  <div v-show="show" class="drawer-overlay" @click.self="close">
    <div class="drawer">
      <button class="close-button" @click="close">×</button>
      <h3>{{ title }}</h3>

      <!-- 配置管理 -->
      <ConfigForm :device="props.device" v-if="type === 'config'" @close="close"/>

      <!-- 网卡配置 -->
      <LogList :device="props.device" v-else-if="type === 'log'" @close="close"/>  

      <!-- 依赖管理 -->
      <DependencyList :device="props.device" v-else-if="type === 'dependency'"/>

      <!-- 网卡配置 -->
      <NetworkConfig :device="props.device" v-else-if="type === 'network'" @close="close"/>

      <!-- 其他类型 -->
      <div v-else>
        <p>未知类型：{{ type }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";

import ConfigForm from './ConfigForm.vue';
import DependencyList from './DependencyList.vue';
import NetworkConfig from './NetworkConfig.vue';
import LogList from './LogList.vue';


// 定义 props 和 emits
const props = defineProps<{
  show: boolean;
  type: 'config' | 'log' | 'dependency' | 'network';
  device: {sn: string; name: string; mac: string; ip: string; status: string; timestamp: string;} | null;
}>();

const emit = defineEmits(['close', 'update']);

const title = computed(() => {
  const deviceName = props.device?.name || '未知设备';
  switch (props.type) {
    case 'config': return `编辑配置：${deviceName}`;
    case 'log': return `日志管理：${deviceName}`;
    case 'dependency': return `依赖管理：${deviceName}`;
    case 'network': return `网卡配置：${deviceName}`;
  }
});

const close = () => emit('close');

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === "Escape") close();
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});

</script>



<style scoped>
.drawer-overlay {
  position: fixed;  /* 使用 fixed 定位，确保覆盖整个屏幕 */
  top: 88px;
  right: 10px;
  bottom: 0;
  left: 298px;
  height: 90.5vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 9999;  /* 确保抽屉在最上层 */
  display: flex;
  justify-content: flex-end;  /* 抽屉从右侧滑出 */
  transition: 0.3s ease-in-out;  /* 添加过渡效果 */
}

.drawer {
  width: 400px;  /* 抽屉宽度 */
  height: 90.5vh;  /* 设置为视口高度，确保占满屏幕 */
  background: white;
  padding: 20px;
  box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);  /* 抽屉左侧阴影 */
  position: relative;
  box-sizing: border-box;
  transform: translateX(0); /* 保证显示状态 */
  right: 0;
  transition: transform 0.8s ease;  /* 滑动效果 */

  overflow-y: auto; /* 添加这个让内容超出时可滚动 */
}

.drawer-overlay .drawer {
  transform: translateX(0);  /* 显示时从右侧滑入 */
}

.close-button {
  position: absolute;
  top: 10px;
  right: 10px;
  border: none;
  background: transparent;
  font-size: 20px;
  cursor: pointer;
}

.close-button:hover {
  color: #f44336;
}
</style>
