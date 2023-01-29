<template>
  <el-card shadow="never" style="border: 0">
    <a-dropdown :trigger="['click']">
      <template #overlay>
        <a-menu>
          <a-menu-item @click="open">打开</a-menu-item>
          <a-sub-menu key="history" title="打开最近文件">
            <a-menu-item>重新打开关闭的文件</a-menu-item>
            <a-menu-divider/>
            <a-menu-item v-for="(item,index) in historyPath" >{{item}}</a-menu-item>
            <a-menu-divider/>
            <a-menu-item>清除最近文件</a-menu-item>
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
  </el-card>
</template>

<script lang="ts" setup>
import {appWindow, WebviewWindow} from '@tauri-apps/api/window'
import {getUUID} from "../../util/uuidUtil";
import storageUtil from "../../util/StorageUtil";
import StorageUtil from "../../util/StorageUtil";
import CherryObjUtil from "../../util/CherryObjUtil";
import {invoke} from "@tauri-apps/api";
import {ref} from "vue"
//最近文件

const historyPath = ref<Array<string>>()


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

const setStorage = (label: string, path: any) => {
  let map:any = storageUtil.get("windowMap", true)
  if (map === null) {
    map = {}
  }
  map[label] = path
  storageUtil.set("windowMap", map)
}
</script>

<style scoped>
.el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}

.el-divider--horizontal {
  width: 240px;
  margin: 1px;
}

.ant-btn-sm{
  font-size: 12px;
}
.ant-dropdown-menu{
  width: 240px;
}

</style>