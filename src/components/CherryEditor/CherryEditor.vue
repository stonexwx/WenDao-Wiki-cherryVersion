<template>
  <el-card class="box-card" shadow="hover">
    <div id="markdown-container" :style="{height : height+'px'}"></div>
  </el-card>
</template>

<script setup>
import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api";
import CherryObjUtil from "../../util/CherryObjUtil";
import StorageUtil from "../../util/StorageUtil";

//窗口自适应
const height = ref(0)
const size = appWindow.innerSize();
const factor = appWindow.scaleFactor();
const getSize=()=> {
  //默认尺寸
  size.then(async Res => {
    height.value = Res.toLogical(await factor).height -27
  })
  //监听窗口大小修改后高度数值按比例修改
  appWindow.onResized(() => {
    const size = appWindow.innerSize();
    size.then(async Res => {
      height.value = Res.toLogical(await factor).height - 27
    })
  });
  //监听窗口所在屏幕dpi大小修改后高度数值按比例修改
  appWindow.onScaleChanged(({payload})=>{
    const size = appWindow.innerSize();
    size.then(async Res => {
      height.value = Res.toLogical(await payload.scaleFactor).height - 27
    })
  })
}

//监听按键处理
document.onkeyup = function (event){

  window.event.preventDefault()
}

onMounted(async () => {

  getSize()

  //打开文件后从缓存中读取前一个窗口从Rust事件获取到的文本地址，这个有改进空间就看Tauri后期有没有优化
  if (StorageUtil.has("filePath",false)) {
    let path = StorageUtil.get("filePath",false)
    console.log(path)
    await invoke("open_file_for_path", {path: path})
        .then(res => {
          console.log(res)
          CherryObjUtil.interface().setMarkdown(res.text)
          StorageUtil.remove("filePath")
        })
  } else {
    CherryObjUtil.interface()
  }
})
</script>

<style scoped>
:deep(#markdown-container) {
  height: v-bind(height) px;
}
</style>