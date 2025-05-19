<template>
  <div class="page-container">
    <h2>设备列表</h2>

    <!-- 设备总数和在线数量 + 搜索框 -->
    <div class="summary-bar">
      <div class="summary-left">
        <div class="summary-item">
          <span class="label">设备总数</span>
          <span class="value">{{ devices.length }}</span>
        </div>
        <div class="separator"></div>
        <div class="summary-item online">
          <span class="label">在线设备</span>
          <span class="value">{{ onlineDeviceCount }}</span>
        </div>
      </div>
      <div class="summary-right">
        <div class="search-container">
          <input
            type="text"
            v-model="searchQuery"
            placeholder="🔍 搜索设备..."
            class="search-input"
          />
          <button
            v-if="searchQuery"
            class="clear-button"
            @click="searchQuery = ''"
            title="清除搜索"
          >
            ×
          </button>
        </div>
          <button class="refresh-button" @click="refreshDevices()" title="手动刷新设备列表">🔄 刷新</button>
      </div>
    </div>

    <!-- 筛选开关 -->
    <div class="filter-bar">
      <label>
        <input type="checkbox" v-model="showOnlyOnline" />
        仅显示在线设备
      </label>
    </div>

    <!-- 设备表格 -->
    <div>
      <table class="config-table">
        <thead>
          <tr>
            <th class="index-column">序号</th>
            <th>设备SN</th>
            <th>设备名称</th>
            <th>网卡MAC</th>
            <th>网卡IP</th>
            <th>设备状态</th>
            <th>发现时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(device, index) in filteredDevices" :key="device.sn">
            <td class="index-column">{{ index + 1 }}</td>
            <td>{{ device.sn }}</td>
            <td>{{ device.name }}</td>
            <td>{{ device.mac }}</td>
            <td>{{ device.ip }}</td>
            <td>
              <span class="status-wrapper">
                <span
                  class="status-dot"
                  :class="device.status === 'Online' ? 'online' : 'offline'"
                ></span>
                <span
                  class="status-text"
                  :class="device.status === 'Online' ? 'online' : 'offline'"
                >
                  {{ device.status }}
                </span>
              </span>
            </td>
            <td>{{ device.timestamp }}</td>
            <td class="action-column">
              <button class="action-button" @click="openDrawer('config', device)">配置</button>
              <button class="action-button" @click="openDrawer('log', device)">日志</button>
              <button class="action-button" @click="openDrawer('dependency', device)">依赖管理</button>
              <button class="action-button" @click="openDrawer('network', device)">网卡配置</button>
              <button class="action-button delete" @click="deleteDevice(device)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 设备配置抽屉组件 -->
    <Drawer 
      :show="drawerState.visible"
      :type="drawerState.type"
      :device="drawerState.device"
      @close="closeDrawer"
      @update="refreshDevices"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import Drawer from '../../components/Drawer.vue';
import { invoke } from '@tauri-apps/api/core';
import { parse, differenceInSeconds } from 'date-fns';



interface Device {
  sn: string;
  name: string;
  mac: string;
  ip: string;
  os: string;
  status: string;
  timestamp: string;
  broadcast_interval: number;
}

interface DrawerState {
  visible: boolean;
  type: 'config' | 'log' | 'dependency' | 'network';  // 操作类型
  device: Device | null;                     // 当前设备
}


const drawerState = reactive<DrawerState>({
  visible: false,
  type: 'config',
  device: null,
});

const openDrawer = (type: DrawerState['type'], device: Device) => {
  drawerState.type = type;
  drawerState.device = device;
  drawerState.visible = true;
};

const closeDrawer = () => {
  drawerState.visible = false;
  drawerState.device = null;
};

const devices = ref<Device[]>([]);
const showOnlyOnline = ref(false);
const searchQuery = ref('');


const normalizedDevices = computed(() => {
  return devices.value.map(device => {
    const reportTime = parse(device.timestamp, 'yyyy-MM-dd HH:mm:ss', new Date());
    const now = new Date();
    const secondsDiff = differenceInSeconds(now, reportTime);

    const interval = device.broadcast_interval || 30;
    const threshold = interval + 3;

    const isOnline = device.status === 'Online' && secondsDiff <= threshold;

    return {
      ...device,
      status: isOnline ? 'Online' : 'Offline'
    };
  });
});



const onlineDeviceCount = computed(() => {
  return normalizedDevices.value.filter(device => device.status === 'Online').length;
});

const filteredDevices = computed(() => {
  let filtered = normalizedDevices.value;

  if (showOnlyOnline.value) {
    filtered = filtered.filter(d => d.status === 'Online');
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(device =>
      device.sn.toLowerCase().includes(query) ||
      device.name.toLowerCase().includes(query) ||
      device.mac.toLowerCase().includes(query) ||
      device.ip.toLowerCase().includes(query)
    );
  }

  return filtered;
});

const refreshDevices = async () => {
  try {
    const data = await invoke('list_devices');
    devices.value = data as any[]; // 如果你知道具体类型可以替换 any
  } catch (err) {
    console.error('获取设备列表失败', err);
  }
};

const deleteDevice = async (device: Device) => {
  try {
    await invoke('delete_device', { sn: device.sn });
    refreshDevices();
  } catch (err) {
    console.error('删除设备失败', err);
  }
};

onMounted(() => {
  refreshDevices();
});

</script>

<style scoped>
.page-container {
  padding: 20px;
}

.summary-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  font-size: 16px;
  font-weight: bold;
  background: linear-gradient(to right, #f0f8ff, #e0efff);
  border-radius: 8px;
  padding: 8px 12px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.summary-left {
  display: flex;
  align-items: center;
}

.summary-right {
  display: flex;
  align-items: center;
}

.summary-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0 8px;
}

.summary-item.online {
  color: #28a745;
}

.summary-item .label {
  font-size: 14px;
  color: #666;
}

.summary-item .value {
  font-size: 18px;
  font-weight: bold;
}

.separator {
  width: 1px;
  height: 40px;
  background-color: #ccc;
  margin: 0 12px;
}

.filter-bar {
  margin-bottom: 10px;
  font-size: 14px;
}

.search-container {
  position: relative;
  display: flex;
  align-items: center;
}

.search-input {
  padding: 8px 12px;
  padding-right: 30px;
  font-size: 14px;
  border-radius: 20px;
  border: 1px solid #ccc;
  outline: none;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
  background-color: #fff;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
}

.search-input:focus {
  border-color: #007bff;
  box-shadow: 0 0 5px rgba(0, 123, 255, 0.3);
}

.clear-button {
  position: absolute;
  right: 10px;
  background: transparent;
  border: none;
  font-size: 18px;
  color: #888;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.clear-button:hover {
  color: #333;
}

.config-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 16px;
}

.config-table th,
.config-table td {
  border: 1px solid #ccc;
  padding: 8px;
}

.config-table th {
  background-color: #f9f9f9;
  text-align: left;
}

.index-column {
  width: 50px;
}

.action-column {
  width: 300px; /* 调整列宽，确保按钮不换行 */
  white-space: nowrap; /* 防止换行 */
}

.action-button {
  padding: 6px 16px;
  margin-right: 10px; /* 增加按钮间距 */
  cursor: pointer;
  border: 1px solid #007bff;
  background-color: #007bff;
  color: #fff;
  border-radius: 4px;
  font-size: 14px;
  transition: background-color 0.3s, transform 0.2s;
}

.action-button:hover {
  background-color: #0056b3;
  transform: scale(1.05);
}

.action-button.delete {
  background-color: #dc3545;
  border-color: #dc3545;
}

.action-button.delete:hover {
  background-color: #c82333;
  border-color: #c82333;
}

.config-table tbody tr:hover {
  background-color: #f0f8ff;
}

.status-wrapper {
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all 0.3s ease;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  transition: background-color 0.3s ease;
}

.status-dot.online {
  background-color: #28a745;
}

.status-dot.offline {
  background-color: #dc3545;
}

.status-text {
  font-weight: bold;
  transition: color 0.3s ease;
}

.status-text.online {
  color: #28a745;
}

.status-text.offline {
  color: #dc3545;
}

.refresh-button {
  margin-left: 12px;
  padding: 6px 12px;
  font-size: 14px;
  background-color: #17a2b8;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.refresh-button:hover {
  background-color: #138496;
}
</style>
