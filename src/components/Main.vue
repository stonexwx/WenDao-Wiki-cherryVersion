<!--主页面-->
<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <Menu/>
      </el-header>

      <el-container>
        <el-aside width="250px">
          <Aside/>
        </el-aside>

        <el-main>
            <CherryEditor/>
        </el-main>

      </el-container>
    </el-container>
    <Dialog @closeDialog="closeDialog" :flag="flag" message="文件还未保存确认关闭吗？" tip="警告"/>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref} from "vue";
import {appWindow} from '@tauri-apps/api/window';
import 'cherry-markdown/dist/cherry-markdown.min.css'
import Dialog from "./Dialog/Dialog.vue";
import CherryEditor from "./CherryEditor/CherryEditor.vue";
import Aside from "./Aside/Aside.vue";
import StorageUtil from "../util/StorageUtil";

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
      StorageUtil.removeJsonKey("windowMap",appWindow.label)
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

:deep(.el-header){
  --el-header-padding:0;
  padding: var(--el-header-padding);
  height: auto;
}

:deep(.el-tabs) {
  --el-tabs-header-height: 48px;
}
</style>