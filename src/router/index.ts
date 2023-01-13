
import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router'

// 静态路由表
const routes: Array<RouteRecordRaw> = [
    {
        // 路由重定向配置
        path: '/',
        redirect: '/Home'
    }, {
        path: '/Home',
        component: () => import('../components/Main.vue')
    }
]

// 路由对象
const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
