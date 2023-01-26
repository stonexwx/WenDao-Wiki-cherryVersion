<template>
  <el-tabs
      v-model="activeName"
      :stretch="true"
      class="demo-tabs"
      tab-position="top">
    <el-tab-pane label="云文件" name="2">云文件</el-tab-pane>
    <el-tab-pane label="大纲" name="1">
      <el-scrollbar :height="height-60+'px'">
        <span v-html="toc"></span>
      </el-scrollbar>
    </el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>

import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import {listen} from "@tauri-apps/api/event";

//默认选项卡
const activeName = ref('1')
//窗口自适应
const height = ref(0)
const size = appWindow.innerSize();
const factor = appWindow.scaleFactor();
const getSize=()=> {
  size.then(async Res => {
    height.value = Res.toLogical(await factor).height - 27
  })
  appWindow.onResized(() => {
    const size = appWindow.innerSize();
    size.then(async Res => {
      height.value = Res.toLogical(await factor).height - 27
    })
  });
}
//侧边目录生成
const toc = ref() //侧边目录

onMounted(()=>{
  getSize()
  listen("toc",(res:any)=>{
    toc.value = res.payload.message
  })
})
</script>

<style scoped>

</style>