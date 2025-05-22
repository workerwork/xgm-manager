<template>
  <div class="topbar">
    <div class="topbar-content">
      <span class="title">XGM管理系统</span>
    </div>
    <div class="topbar-user">
      <span class="user-info">{{ user.name }}</span>
    </div>
    <button class="logout-button" @click="handleLogout">登出</button>
  </div>
</template>


<script lang="ts" setup>
import { useAuth } from '@/composables/useAuth';

const { user, logout } = useAuth();

const handleLogout = (): void => {
  logout();
};
</script>


<style scoped>
.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 80px;
  background-color: #333;
  color: white;
  padding: 0 20px;
}

.topbar-content {
  display: flex;
  align-items: center;
}

.topbar-user {
  display: flex;
  align-items: center;
  margin-left: auto; /* ✅ 让内容靠右 */
  padding: 10px;
}

.title {
  font-size: 32px; /* 设置文字大小 */
  font-weight: bold; /* 加粗文字 */
}

.user-info {
  margin-left: 20px;
  font-size: 18px;
}

.logout-button {
  background-color: #ff4d4d;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
}

.logout-button:hover {
  background-color: #ff1a1a;
}
</style>
