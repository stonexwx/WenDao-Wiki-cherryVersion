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
  size.then(async Res => {
    height.value = Res.toLogical(await factor).height -27
  })
  appWindow.onResized(() => {
    const size = appWindow.innerSize();
    size.then(async Res => {
      height.value = Res.toLogical(await factor).height - 27
    })
  });
}

onMounted(()=>{

  getSize()
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