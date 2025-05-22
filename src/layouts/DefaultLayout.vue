<template>
  <div class="layout">
    <div class="topbar">
      <Topbar />
    </div>
    <div class="main-container">  <!-- 新增包裹容器 -->
      <div class="sidebar">
        <Sidebar />
      </div>     
      <div class="content">
        <router-view />
      </div>
    </div>
  </div>
</template>


<script setup lang="ts">
import Topbar from './Topbar.vue';
import Sidebar from './Sidebar.vue';
</script>


<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  height: 98.4vh;
}

.topbar {
  height: 80px;
  background-color: #333;
  color: white;
  flex-shrink: 0; /* 防止顶部栏被压缩 */
}

.main-container {
  flex: 1;
  display: flex;
  overflow: hidden;
  height: calc(100% - 80px);  /* 减去 topbar 高度 */
}

.sidebar {
  width: 250px;   /* 给侧边栏固定宽度 */
  flex-shrink: 0; /* 防止侧边栏被压缩 */
  overflow-y: auto; /* 如果内容过长允许滚动 */
  background: linear-gradient(180deg, #2c3e50 0%, #34495e 100%);
  color: white;
  padding: 20px;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.2);
}

.content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background-color: #f4f4f4;
}

</style>