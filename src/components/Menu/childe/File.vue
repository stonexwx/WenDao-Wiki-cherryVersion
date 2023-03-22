<template>
  <a-dropdown :trigger="['click']">
    <template #overlay>
      <a-menu>
        <a-menu-item @click="open">打开</a-menu-item>
        <a-sub-menu key="history" title="打开最近文件" @mouseover.once="openHistory">
          <a-menu-item>重新打开关闭的文件</a-menu-item>
          <a-menu-divider/>
          <a-menu-item v-for="(val,key,index) in history " :key="key" >{{val.path}}</a-menu-item>
          <a-menu-divider/>
          <a-menu-item @click="clear">清除所有记录</a-menu-item>
        </a-sub-menu>
        <a-menu-divider/>
        <a-menu-item @click="newWindow">新建</a-menu-item>
        <a-menu-divider/>
        <a-menu-item @click="save">保存</a-menu-item>
        <a-menu-item @click="saveAs">另存为</a-menu-item>
        <a-menu-divider/>
        <a-sub-menu key="export" title="导出">
          <a-menu-item @click="exportHTML">HTML</a-menu-item>
          <a-menu-divider style="width: 240px"/>
          <a-menu-item @click="exportImage">图像</a-menu-item>
          <a-menu-divider/>
          <a-menu-item @click="CherryObjUtil.interface().export()">PDF</a-menu-item>
        </a-sub-menu>
      </a-menu>
    </template>
    <a-button type="text" size="small">
      文件
    </a-button>
  </a-dropdown>
</template>

<script lang="ts" setup>
import CherryObjUtil from "../../../util/CherryObjUtil";
import {invoke} from "@tauri-apps/api";
import {appWindow, WebviewWindow} from "@tauri-apps/api/window";
import StorageUtil from "../../../util/StorageUtil";
import {getUUID} from "../../../util/uuidUtil";
import storageUtil from "../../../util/StorageUtil";
import {ref} from "vue";
import {json} from "stream/consumers";

/**
 * 打开文件
 * @returns {Promise<void>}
 */
const open = async () => {

  if (CherryObjUtil.interface().getValue() === "") {
    await invoke("open").then(async (res:any) => {
      CherryObjUtil.interface().setMarkdown(res.text)
      setStorage(appWindow.label, res.path)
      await appWindow.setTitle(res.name)
    })
  } else {
    await invoke("choose_file").then(async (res:any) => {
      StorageUtil.set("filePath", res.path)
      let uuid = getUUID()
      const webview = new WebviewWindow(uuid, {
        url: '/Home',
        title: res.name
      })
      await webview.once('tauri://created', async function () {
        setStorage(uuid, res.path)
      })
      await webview.once('tauri://error', function (e) {
        // an error occurred during webview window creation
        ElMessage.error("创建失败：" + e)
      })
    })
  }
}

/**
 * 创建新的窗口
 * @returns {Promise<void>}
 */
const newWindow = async () => {
  let uuid = getUUID()
  const webview = new WebviewWindow(uuid, {
    url: '/Home',
    title: "Cherry"
  })
  await webview.once('tauri://error', function (e) {
    // an error occurred during webview window creation
    ElMessage.error("创建失败：" + e)
  })
}

/**
 * 文件保存
 * @returns {Promise<void>}
 */
const save = async () => {
  if (StorageUtil.session.get("save",false)==="true") {
    let map:any = storageUtil.get("windowMap", true)
    let path = map[appWindow.label]
    if (path===undefined){
      await saveAs()
    }else {
      await invoke("save",{text:CherryObjUtil.interface().getMarkdown(),path:path})
          .then((res:any) =>{
            ElMessage.success(res)
          })
          .catch((err:any)=>{
            ElMessage.error(err)
            console.log(err)
          })
    }

  }
}

/**
 * 另存为
 */
const saveAs = async ()=>{

  await invoke("save_as",{text:CherryObjUtil.interface().getMarkdown()})
      .then((res:any) =>{
        ElMessage.success(res)
      })
}

/**
 * 导出
 */

//html
const exportHTML = async ()=>{

  await invoke("save_as",{text:CherryObjUtil.interface().getHtml(true)})
      .then((res:any) =>{
        ElMessage.success(res)
      })
}

//img
const exportImage = async ()=>{

  CherryObjUtil.interface().export('img')

}


/**
 * 获取历史记录
 */
const history = ref()
const openHistory = async ()=>{
  let historyJson:string = await invoke("get_open_history")
  history.value = JSON.parse(historyJson)
  console.log(history)
}

/**
 * 清楚历史记录
 */
const clear = async ()=>{
  await invoke("del_open_history")
     .catch((err:any) =>{
        ElMessage.error(err)
      })
}

/**
 * 缓存窗口map
 * @param label
 * @param path
 */
const setStorage = (label: string, path: any) => {
  let map:any = storageUtil.get("windowMap", true)
  if (map === null) {
    map = {}
  }
  map[label] = path
  storageUtil.set("windowMap", map)
}
</script>

<style lang="sass" scoped>
@import "../sass/btn_and_menu.sass"
.ant-btn-sm
  @include ant-btn-sm

.ant-dropdown-menu
  @include ant-dropdown-menu

</style>