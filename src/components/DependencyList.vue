<template>
  <div class="container">
    <div class="search-container">
      <input
        type="text"
        v-model="searchQuery"
        placeholder="🔍 搜索包..."
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
    <button class="action-button" @click="showUpgrade()">升级</button>
  </div>
  <table class="config-table">
    <thead>
      <tr>
        <th class="index-column">序号</th>
        <th>包名</th>
        <th>版本</th>
        <th>操作</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(pkg, index) in pagedPackages" :key="index">
        <td class="index-column">{{ (currentPage - 1) * pageSize + index + 1 }}</td>
        <td>{{ pkg.name }}</td>
        <td>{{ pkg.version }}</td>
        <td class="action-column">
          <button class="action-button" @click="showDetails(pkg)">详情</button>
          <!-- <button class="action-button" @click="showUpgrade(pkg)">升级</button> -->
        </td>
      </tr>
    </tbody>
  </table>
  <div class="pagination">
    <button @click="prevPage" :disabled="currentPage === 1">上一页</button>
    <span>第 {{ currentPage }} / {{ totalPages }} 页</span>
    <button @click="nextPage" :disabled="currentPage === totalPages">下一页</button>
  </div>
  <PackageDetailModal
    :show="showModal"
    :pkg="selectedPackage"
    @close="closeModal"
  />
  <PackageUpgradeModal
    :show="showUpgradeModal"
    :pkg="upgradePackage"
    :device="device"
    :selected-file-path="selectedFilePath"
    @close="closeUpgradeModal"
    @upload="refreshPackages"
  />
</template>

<script setup lang="ts">
import { computed, watch, ref} from 'vue';
import PackageDetailModal from '../components/PackageDetailModal.vue';
import PackageUpgradeModal from '../components/PackageUpgradeModal.vue';
import { invoke } from "@tauri-apps/api/core";


interface PackageInfo {
  name: string;
  version: string;
}

const searchQuery = ref('');
const currentPage = ref(1);
const pageSize = ref(14);
const packages = ref<PackageInfo[]>([]);
const loading = ref(false);
const showModal = ref(false);
const selectedPackage = ref<[string, string, string, string]>(["", "", "", ""]);
const showUpgradeModal = ref(false);
const upgradePackage = ref<{ name: string; version: string }>({ name: "", version: "" });
const selectedFilePath = ref<string | null>(null);

// 接收父组件传递的设备信息
const props = defineProps<{
  device: { sn: string; name: string; mac: string; ip: string; status: string; timestamp: string } | null;
}>();

const closeModal = () => showModal.value = false;
const closeUpgradeModal = () => showUpgradeModal.value = false;



// 获取包列表
const refreshPackages = async () => {
  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置');
    alert('设备信息不完整，无法获取配置');
    return;
  }

  try {
    const res = await invoke<PackageInfo[]>("get_package_list", { 
      deviceIp: props.device.ip,
    });
    packages.value = res;
  } catch (err) {
    console.error(err);
  }
};


const filteredPackages = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  return packages.value.filter(pkg => pkg.name.toLowerCase().includes(query));
});

const pagedPackages = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return filteredPackages.value.slice(start, start + pageSize.value);
});

const totalPages = computed(() => Math.max(1, Math.ceil(filteredPackages.value.length / pageSize.value)));

const nextPage = () => {
  if (currentPage.value < totalPages.value) currentPage.value++;
};

const prevPage = () => {
  if (currentPage.value > 1) currentPage.value--;
};


// 获取包详情
const showDetails = async (pkg: PackageInfo) => {
  loading.value = true;
  // 检查设备信息是否完整，若不完整则直接返回
  if (!props.device?.ip) {
    console.warn('设备信息不完整，无法获取配置');
    alert('设备信息不完整，无法获取配置');
    return;
  }
  try {
    const res = await invoke<{ location: string; dependencies: string[] }>(
      "get_package_detail",
      {
        pkgName: pkg.name,
        deviceIp: props.device.ip,
       }
    );
    selectedPackage.value = [pkg.name, pkg.version, res.location, res.dependencies.join(", ")];
    showModal.value = true;
  } catch (err) {
    console.error(err);
  } finally {
    loading.value = false;
  }
};


// 升级包
// const showUpgrade = (pkg: PackageInfo) => {
//   upgradePackage.value = { name: pkg.name, version: pkg.version };
//   selectedFilePath.value = null; // 清空之前选择的文件路径
//   showUpgradeModal.value = true;
// };

const showUpgrade = () => {
  selectedFilePath.value = null; // 清空之前选择的文件路径
  showUpgradeModal.value = true;
};


watch(() => props.device, async (newDevice) => {
  if (newDevice?.ip) {
    await refreshPackages();
  }
}, { immediate: true }); // immediate: true 表示组件加载时就会调用一次

// 监听搜索时自动跳到第一页
watch(searchQuery, () => {
  currentPage.value = 1;
});

</script>

<style scoped>
.container {
  display: flex;
  align-items: center;  /* 垂直居中对齐 */
  gap: 10px;  /* 搜索框和按钮之间的间距 */
}

.search-container {
  position: relative;
  width: 80%;
  max-width: 300px;
}

.search-input {
  width: 100%;
  padding: 8px 30px 8px 10px; /* 给右边留出空隙给清除按钮 */
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box;
}

.clear-button {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  border: none;
  background: transparent;
  color: #888;
  font-size: 16px;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.clear-button:hover {
  color: #000;
}

.config-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 14px;
}

.config-table th,
.config-table td {
  padding: 6px 8px;
  text-align: left;
  white-space: nowrap;
}


.index-column {
  width: 40px;
}

.action-column {
  width: 120px;
}

.config-table th:nth-child(2),
.config-table td:nth-child(2) {
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.config-table th:nth-child(3),
.config-table td:nth-child(3) {
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.config-table th:nth-child(4),
.config-table td:nth-child(4) {
  max-width: 50px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.action-button {
  padding: 4px 12px;
  margin-right: 8px;
  cursor: pointer;
  border: 1px solid #007bff;
  background-color: #007bff;
  color: #fff;
  border-radius: 4px;
  font-size: 14px;
}

.action-button:hover {
  background-color: #0056b3;
}

.config-table tbody tr:hover {
  background-color: #f0f8ff;
}

.pagination {
  margin-top: 16px;
  text-align: center;
  position: absolute;
  right: 20px;
  bottom: 100px;
}

.pagination button {
  margin: 0 8px;
  padding: 6px 12px;
  font-size: 14px;
  cursor: pointer;
}

.pagination button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}


</style>
