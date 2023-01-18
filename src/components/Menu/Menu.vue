<template>
  <el-card shadow="never" style="border: 0">
    <el-menu
        class="el-menu-demo"
        mode="horizontal"
        style="height: 25px"
    >
      <el-sub-menu index="1">
        <template #title>文件</template>
        <el-menu-item index="1-1" @click="open">打开</el-menu-item>
        <el-divider></el-divider>
        <el-menu-item index="1-2" @click="newWindow">新建</el-menu-item>
        <el-divider></el-divider>
        <el-menu-item index="1-3" >保存</el-menu-item>
        <el-menu-item index="1-3">另存为</el-menu-item>
        <el-divider></el-divider>
        <el-sub-menu index="1-4">
          <template #title>导出</template>
          <el-menu-item index="1-4-1">HTML</el-menu-item>
          <el-menu-item index="1-4-2">图像</el-menu-item>
          <el-menu-item index="1-4-3">PDF</el-menu-item>
        </el-sub-menu>
      </el-sub-menu>

    </el-menu>
  </el-card>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window'
import Cherry from "cherry-markdown";
import { invoke } from '@tauri-apps/api/tauri'
import { WebviewWindow } from '@tauri-apps/api/window'
import {getUUID} from "../../util/uuidUtil";
import storageUtil from "../../util/StorageUtil";

const props = defineProps({
  cherry: Cherry
})

const open= async ()=> {
  await invoke("open").then(async res => {
    if (props.cherry.getValue() === "") {

      // props.cherry.setMarkdown(res.text)
      console.log(props.cherry.cherryDom)
      sessionStorage.setItem("save","true")
      setStorage(appWindow.label,res.path)
      await appWindow.setTitle(res.name)

    } else {
      let uuid = getUUID()
      const webview = new WebviewWindow(uuid, {
        url: '/Home',
        title: res.name
      })
      await webview.once('tauri://created', async function () {
        localStorage.setItem("text", res.text)
        setStorage(uuid,res.path)
      })
      await webview.once('tauri://error', function (e) {
        // an error occurred during webview window creation
        ElMessage.error("创建失败："+e)
      })
    }
  })
}

const newWindow = async ()=>{
  let uuid = getUUID()
  const webview = new WebviewWindow(uuid, {
    url: '/Home',
    title: "Cherry"
  })
  await webview.once('tauri://error', function (e) {
    // an error occurred during webview window creation
    ElMessage.error("创建失败："+e)
  })
}

const save = async ()=>{
  if(sessionStorage.getItem("save")==="true"){

  }
}

const setStorage =  (label,path)=>{
  let map = storageUtil.get("windowMap",true)
  if (map === null){
    map = {}
  }
  map[label] = path
  storageUtil.set("windowMap",map)
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
  margin:1px;
}

</style>