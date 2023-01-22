<template>
  <el-card class="box-card" shadow="hover">
    <div id="markdown-container" :style="{height : height+'px'}"></div>
  </el-card>
</template>

<script setup>
import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import Cherry from "cherry-markdown";
import {invoke} from "@tauri-apps/api";
import CherryObjUtil from "../../util/CherryObjUtil";

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

onMounted(()=>{

  getSize()

  //打开文件后从缓存中读取前一个窗口从Rust事件获取到的文本信息，这个有改进空间就看Tauri后期有没有优化
  if(localStorage.getItem("text")!==null &&localStorage.getItem("text")!==""){
    CherryObjUtil.interface().setMarkdown(localStorage.getItem("text"))
    localStorage.removeItem("text")
  }
  CherryObjUtil.interface()

})
</script>

<style scoped>
:deep(#markdown-container) {
  height: v-bind(height) px;
}
</style>