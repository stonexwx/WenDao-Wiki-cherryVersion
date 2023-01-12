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
          <el-dropdown-item>Action 2</el-dropdown-item>
          <el-dropdown-item>Action 3</el-dropdown-item>
          <el-dropdown-item>Action 4</el-dropdown-item>
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

const props = defineProps({
  cherry: Cherry
})

const open= async ()=> {

  if(props.cherry.getValue() === ""){
    let obj = await invoke("open")
    props.cherry.setMarkdown(obj.text)
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

</style>