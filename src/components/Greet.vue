<script setup>
import Cherry from 'cherry-markdown'
import 'cherry-markdown/dist/cherry-markdown.min.css'
import {onMounted, ref} from "vue";
function init(){
  new Cherry({
    id: 'markdown-container',
    value: '# welcome to cherry editor!',
    toolbars: {
      showToolbar: false,
    },
    previewer: {
      enablePreviewerBubble: true,
    }
  })

}
onMounted(()=>{
  init()
})
</script>

<template>
  <div id="markdown-container" style="height:auto"></div>
</template>
