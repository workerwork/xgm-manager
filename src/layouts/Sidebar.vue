<template>
  <div class="sidebar">
    <nav class="sidebar-nav">
      <header class="sidebar-header">
        <h2 class="title">导航菜单</h2>
      </header>

      <ul class="nav-list" v-if="isLoggedIn()">
        <li v-for="item in navItems" :key="item.text" class="nav-item">
          <!-- 顶级菜单 -->
          <div class="nav-link" @click="toggleExpand(item)">
            <span class="nav-icon">{{ item.icon }}</span>
            <span class="nav-text">{{ item.text }}</span>
            <span class="expand-indicator">
              {{ item.children ? (item.expanded ? '▼' : '▶') : '' }}
            </span>
          </div>

          <!-- 子菜单，改用 v-if 确保不会渲染未展开的部分 -->
          <ul v-if="item.children && item.expanded" class="sub-nav-list">
            <li v-for="sub in item.children" :key="sub.path" class="nav-item">
              <!-- 包裹 router-link 防止干扰展开状态 -->
              <div class="nav-link sub-link-wrapper">
                <router-link 
                  :to="sub.path"
                  class="sub-link"
                  active-class="active"
                  exact-active-class="exact-active"
                  :title="sub.title"
                >
                  <span class="nav-icon">{{ sub.icon }}</span>
                  <span class="nav-text">{{ sub.text }}</span>
                </router-link>
              </div>
            </li>
          </ul>

        </li>
      </ul>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAuth } from '@/composables/useAuth';

// 定义导航菜单项的类型
interface NavChild {
  path: string;
  icon: string;
  text: string;
  title: string;
}

interface NavItem {
  text: string;
  icon: string;
  expanded: boolean;
  children?: NavChild[];
}

const { isLoggedIn } = useAuth();

const navItems = ref<NavItem[]>([
  {
    text: '设备管理',
    icon: '🖥️',
    expanded: true,
    children: [
      { path: '/device', icon: '📋', text: '设备列表', title: '查看所有设备' },
    ]
  },
  {
    text: '终端管理',
    icon: '🖥️',
    expanded: true,
    children: [
      { path: '/jupyter', icon: '🧪', text: 'Jupyter', title: 'Jupyter Notebook' },
    ]
  },
  // 你可以按需打开以下菜单
  // {
  //   text: '用户权限管理',
  //   icon: '👤',
  //   expanded: false,
  //   children: [
  //     { path: '/users/list', icon: '👥', text: '用户列表', title: '管理用户' },
  //     { path: '/users/roles', icon: '🛡️', text: '角色分配', title: '设置用户角色' },
  //     { path: '/users/logs', icon: '📝', text: '登录日志', title: '查看登录日志' },
  //   ]
  // },
]);

function toggleExpand(item: NavItem) {
  if (!item.children) return;
  item.expanded = !item.expanded;
}
</script>


<style scoped>
:root {
  --sidebar-bg: #2c3e50;
  --sidebar-text: #ecf0f1;
  --sidebar-hover: rgba(255,255,255,0.1);
  --sidebar-active: #3498db;
  --sidebar-border: rgba(255,255,255,0.2);
}

.sidebar {
  background: var(--sidebar-bg);
  color: var(--sidebar-text);
  min-height: 100%;
  padding: 20px;
  box-sizing: border-box;
}

.sidebar-header {
  padding: 0 15px 15px;
  margin-bottom: 20px;
  border-bottom: 1px solid var(--sidebar-border);
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  letter-spacing: 0.5px;
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  margin-bottom: 8px;
}

.nav-link {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-radius: 6px;
  color: inherit;
  text-decoration: none;
  cursor: pointer;
  transition: background 0.25s ease, transform 0.2s ease;
}

.nav-link:hover {
  background: var(--sidebar-hover);
  transform: translateX(4px);
}

.sub-nav-list {
  list-style: none;
  padding-left: 20px;
  margin-top: 6px;
}

.sub-link-wrapper {
  padding: 10px 16px;
  border-radius: 6px;
  transition: background 0.2s ease;
}

.sub-link-wrapper:hover {
  background: var(--sidebar-hover);
}

.sub-link {
  display: flex;
  align-items: center;
  color: inherit;
  text-decoration: none;
}

.nav-icon {
  margin-right: 8px;
  font-size: 1.1em;
}

.nav-text {
  flex: 1;
  font-size: 0.95rem;
}

.expand-indicator {
  font-size: 0.9em;
  margin-left: 8px;
}

/* 激活状态样式 */
.active,
.exact-active {
  background: var(--sidebar-active) !important;
  font-weight: 500;
  position: relative;
}

.active::before {
  content: '';
  position: absolute;
  left: -20px;
  top: 0;
  height: 100%;
  width: 4px;
  background: var(--sidebar-text);
}

/* 响应式优化 */
@media (max-width: 768px) {
  .sidebar {
    padding: 15px;
  }
  .nav-link {
    padding: 10px 14px;
  }
  .title {
    font-size: 1.3rem;
  }
}
</style>
