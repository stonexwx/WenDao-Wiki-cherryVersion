<!--主页面-->
<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <Menu :cherry="cre"/>
      </el-header>

      <el-container>
        <el-aside width="250px">
          <el-tabs :stretch="true"
                   class="demo-tabs"
                   tab-position="top">
            <el-tab-pane label="云文件">云文件</el-tab-pane>
            <el-tab-pane label="大纲">
              <el-scrollbar :height="height-60+'px'">
              <span v-html="toc"></span>
              </el-scrollbar>
            </el-tab-pane>
          </el-tabs>
        </el-aside>

        <el-main>
          <el-card class="box-card" shadow="hover">
            <CherryEditor/>
          </el-card>
        </el-main>
      </el-container>
    </el-container>
    <Dialog @closeDialog="closeDialog" :flag="flag" message="文件还未保存确认关闭吗？" tip="警告"/>
  </div>
</template>

<script setup>
import {onMounted, ref} from "vue";
import {appWindow} from '@tauri-apps/api/window';
import 'cherry-markdown/dist/cherry-markdown.min.css'
import Dialog from "./Dialog/Dialog.vue";
import CherryEditor from "./CherryEditor/CherryEditor.vue";

//弹出框
let flag  = ref(false)

const closeDialog = () => {
  flag.value=false
}
onMounted(() => {
  //监测窗口关闭
  appWindow.listen('tauri://close-requested',async ()=>{
    if(sessionStorage.getItem("save")==="true"){
      flag.value = true
    }else {
        await appWindow.close()
    }
  })
})

</script>

<style scoped>
:deep(.el-card__body) {
  padding: 0;
}

:deep(.el-main) {
  padding: 0;
}

:deep(#markdown-container) {
  height: v-bind(height) px;
}

:deep(.el-tabs) {
  --el-tabs-header-height: 48px;
}
:deep(.el-header){
  --el-header-padding:0;
  padding: var(--el-header-padding);
  height: auto;
}

</style>