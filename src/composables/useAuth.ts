// src/composables/useAuth.ts
import { ref } from 'vue';
import { useRouter } from 'vue-router';


interface User {
    name: string;
}

const user = ref<User>({ name: localStorage.getItem('username') || '未登录' });

export function useAuth() {
    const router = useRouter();

    const login = (username: string, token: string) => {
        localStorage.setItem('username', username);
        localStorage.setItem('token', token);
        user.value.name = username;
    };   

    const logout = () => {
        localStorage.removeItem('username');
        localStorage.removeItem('token');
        user.value.name = '未登录';
        // window.location.href = '/login'; // router.replace('/login') 不可用
        router.replace('/login')
    };

    const isLoggedIn = () => {
        return user.value.name !== '未登录';
    };

    return {
        user,
        login,
        logout,
        isLoggedIn
    };
}
