<template>
  <div class="login-wrapper">
    <div class="login-card">
      <h2>欢迎登录</h2>
      <form @submit.prevent="handleLogin">
        <div class="form-group">
          <label for="username">用户名</label>
          <div class="input-icon">
            <i class="icon">👤</i>
            <input type="text" id="username" v-model="username" required />
          </div>
        </div>
        <div class="form-group">
          <label for="password">密码</label>
          <div class="input-icon">
            <i class="icon">🔒</i>
            <input type="password" id="password" v-model="password" required />
          </div>
        </div>
        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>
        <button type="submit" :disabled="isLoading">
          {{ isLoading ? '登录中...' : '立即登录' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '@/composables/useAuth';

export default defineComponent({
  setup() {
    const username = ref('');
    const password = ref('');
    const errorMessage = ref('');
    const isLoading = ref(false);
    const router = useRouter();
    const { login } = useAuth();

    // 模拟登录
    const handleLogin = async () => {
      isLoading.value = true;
      errorMessage.value = '';
      
      try {
        const token = await invoke<string>('login', {
          username: username.value,
          password: password.value
        });

        localStorage.setItem('token', token); // 模拟登录状态
        localStorage.setItem('username', username.value); // 模拟用户名

        login(username.value, token); // 使用封装的 login 方法
        router.replace('/device');

      } catch (error: any) {
        errorMessage.value = error.toString().replace('Error: ', '');
      } finally {
        isLoading.value = false;
      }
    };

    return {
      username,
      password,
      errorMessage,
      isLoading,
      handleLogin,
    };
  }
});
</script>

<style scoped>
.login-wrapper {
  height: 100vh;
  background: linear-gradient(135deg, #74ebd5 0%, #9face6 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.login-card {
  background: white;
  padding: 30px 40px;
  border-radius: 16px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
  width: 320px;
  max-width: 90%;
}

h2 {
  text-align: center;
  margin-bottom: 24px;
  color: #333;
}

.form-group {
  margin-bottom: 20px;
}

label {
  font-size: 14px;
  font-weight: 500;
  display: block;
  margin-bottom: 6px;
}

.input-icon {
  position: relative;
  width: 100%;
  overflow: hidden; /* 防止溢出 */
  box-sizing: border-box;
}

.input-icon input {
  width: 100%;
  box-sizing: border-box; /* ✅ 关键点：让 padding 不撑大元素 */
  padding-left: 32px; /* 给图标留位置 */
  height: 40px;
  border: 1px solid #ccc;
  border-radius: 8px;
  transition: all 0.2s ease;
  font-size: 14px;
}

.input-icon input:focus {
  border-color: #007bff;
  outline: none;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 16px;
  color: #888;
}

button {
  width: 100%;
  height: 40px;
  background-color: #007bff;
  color: white;
  font-weight: bold;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

button:hover:not(:disabled) {
  background-color: #0056b3;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.error-message {
  color: #dc3545;
  font-size: 13px;
  margin-bottom: 15px;
  text-align: center;
}
</style>
