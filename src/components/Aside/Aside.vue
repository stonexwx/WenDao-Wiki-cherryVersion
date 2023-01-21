<template>
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
</template>

<script setup>


</script>

<style scoped>

</style>