<template>
  <el-dialog
      v-model="dialogVisible"
      :title="props.tip"
      width="30%"
      :before-close="handleClose"
  >
    <span>{{props.message}}</span>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancel">取消</el-button>
        <el-button type="danger" @click="close">
          确认
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import {computed, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";

interface Props{
  message:string,
  flag:boolean,
  tip:string
}
const emit = defineEmits(['closeDialog'])
const props = defineProps<Props>()
const dialogVisible =computed(() => props.flag)
const handleClose = (done: () => void) => {
  done()
}

const close = ()=>{
  appWindow.close()
}

const cancel = () => {
  emit('closeDialog')
}
</script>

<style scoped>

</style>