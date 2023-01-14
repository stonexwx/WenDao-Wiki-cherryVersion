<template>
  <el-card shadow="never" style="border: 0">
<!--    <el-dropdown size="large" trigger="click">-->
<!--      <el-button-->
<!--          text-->
<!--          size="small"-->
<!--      >-->
<!--        文件-->
<!--      </el-button>-->
<!--      <template #dropdown>-->
<!--        <el-dropdown-menu>-->
<!--          <el-dropdown-item @click="open">打开</el-dropdown-item>-->
<!--          <el-divider></el-divider>-->
<!--          <el-dropdown-item>新建</el-dropdown-item>-->
<!--          <el-divider></el-divider>-->
<!--          <el-dropdown-item>保存</el-dropdown-item>-->
<!--          <el-dropdown-item>另存为</el-dropdown-item>-->
<!--          <el-divider></el-divider>-->
<!--          <el-dropdown-item>-->
<!--            <span>个人中心</span>-->
<!--            <el-popover placement="left-start" trigger="hover" :offset="15">-->
<!--              <el-dropdown-item>-->
<!--                HTML-->
<!--              </el-dropdown-item>-->
<!--            </el-popover>-->
<!--          </el-dropdown-item>-->
<!--          <el-divider></el-divider>-->
<!--          <el-dropdown-item>关闭</el-dropdown-item>-->
<!--        </el-dropdown-menu>-->
<!--      </template>-->
<!--    </el-dropdown>-->
    <el-menu
        :default-active="activeIndex"
        class="el-menu-demo"
        mode="horizontal"
        @select="handleSelect"
        style="height: 30px"
    >
      <el-sub-menu index="1">
        <template #title>文件</template>
        <el-menu-item index="1-1" @click="open">打开</el-menu-item>
        <el-divider></el-divider>
        <el-menu-item index="1-2">新建</el-menu-item>
        <el-divider></el-divider>
        <el-menu-item index="1-3">保存</el-menu-item>
        <el-menu-item index="1-3">另存为</el-menu-item>
        <el-divider></el-divider>
        <el-sub-menu index="1-4">
          <template #title>item four</template>
          <el-menu-item index="2-4-1">item one</el-menu-item>
          <el-menu-item index="2-4-2">item two</el-menu-item>
          <el-menu-item index="2-4-3">item three</el-menu-item>
        </el-sub-menu>
      </el-sub-menu>

    </el-menu>
  </el-card>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window'
import { ref } from 'vue'
import Cherry from "cherry-markdown";
import { invoke } from '@tauri-apps/api/tauri'
import { WebviewWindow } from '@tauri-apps/api/window'
import {getUUID} from "../../util/uuidUtil";
import { StarFilled } from '@element-plus/icons-vue'

const props = defineProps({
  cherry: Cherry
})

const open= async ()=> {
  let obj = await invoke("open")
  if(props.cherry.getValue() === ""){
    props.cherry.setMarkdown(obj.text)
    await appWindow.setTitle(obj.name)
  }else {
    let uuid = getUUID()
    const webview = new WebviewWindow(uuid, {
      url: '/Home',
      title: obj.name
    })
    await webview.once('tauri://created', async function () {
      localStorage.setItem("text",obj.text)
    })
    await webview.once('tauri://error', function (e) {
      // an error occurred during webview window creation
      console.log(e)
    })
  }



}
</script>

<style scoped>
.el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}
.el-divider--horizontal{
  width: 240px;
  margin:0;
}
</style>