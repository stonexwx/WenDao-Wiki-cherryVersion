<template>
  <el-card shadow="never" style="border: 0">
    <el-dropdown size="large" trigger="click">
      <el-button
          text
          size="small"
      >
        文件
      </el-button>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item @click="open">
            打开
          </el-dropdown-item>
          <el-divider></el-divider>
          <el-dropdown-item>新建</el-dropdown-item>
          <el-divider></el-divider>
          <el-dropdown-item>保存</el-dropdown-item>
          <el-dropdown-item>另存为</el-dropdown-item>
          <el-divider></el-divider>
          <el-dropdown-item>关闭</el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
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
:deep(.el-divider--horizontal){
  width: 240px;
  margin:0;
}
</style>