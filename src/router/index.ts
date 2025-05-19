import { createRouter, createWebHashHistory } from 'vue-router';
import Login from '@/views/Login.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import DeviceList from '@/views/devices/DeviceList.vue';
import Jupyter from '@/views/terminals/Jupyter.vue';


const routes = [
  // { path: '/', redirect: '/login' }, // 默认跳转登录页
  {
    path: '/login',
    name: 'Login',
    component: Login
  },
  {
    path: '/',
    component: DefaultLayout,
    meta: { requiresAuth: true },
    children: [
      {
        path: '',
        redirect: '/device'
      },
      {
        path: '/device',
        name: 'DeviceList',
        component: DeviceList
      },
      {
        path: '/jupyter',
        name: 'jupyter',
        component: Jupyter,
      },
    ]
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

// 🔐 登录状态导航守卫
router.beforeEach((to, _from, next) => {
  const token = localStorage.getItem('token');
  const isAuthenticated = !!token;

  if (to.name !== 'Login' && !isAuthenticated) {
    next({ name: 'Login' });
  } else if (to.name === 'Login' && isAuthenticated) {
    next({ name: 'DeviceList' });
  } else {
    next(); // 放行
  }
});

export default router;
