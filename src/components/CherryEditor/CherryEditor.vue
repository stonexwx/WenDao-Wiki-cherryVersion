<template>
  <el-card class="box-card" shadow="hover">
    <div id="markdown-container" :style="{height : height+'px'}"></div>
  </el-card>
</template>

<script setup>
//窗口自适应
import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import Cherry from "cherry-markdown";
import {invoke} from "@tauri-apps/api";

const height = ref(0)
const size = appWindow.innerSize();
const factor = appWindow.scaleFactor();
const cre = ref()
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
//初始化窗口
function init() {
  getSize()
  //cherry 监听
  const callbacks = {
    //用户输入字符监听
    afterChange: (text, html) => {
      sessionStorage.setItem("save","ture")
      createToc(cherry)
    },
  }
  let text=""
  if(localStorage.getItem("text")!==null &&localStorage.getItem("text")!==""){
    text = localStorage.getItem("text")
    localStorage.removeItem("text")
  }

  let cherry = new Cherry({
    id: 'markdown-container',
    value: text,
    previewer: {
      enablePreviewerBubble: true,
    },
    callback: {
      afterChange: callbacks.afterChange,
    }
  })

  if(text!==""){
    sessionStorage.setItem("save","true")
  }else {
    sessionStorage.setItem("save","false")
  }
  cre.value = cherry
  createToc(cherry)
}

//侧边目录生成
const toc = ref() //侧边目录
let tocOld

//通过rust生成目录结构
const jsInvoke = (headerList)=>{
  invoke('create_toc',{json:headerList})
      .then(res =>{
        typeof res === "string" ? toc.value = res : ""
      })
}

//创建目录
const createToc = (cherry)=> {
  let headerList = JSON.stringify(getToc(cherry))
  if(tocOld===undefined||tocOld===null){
    tocOld=headerList
    jsInvoke(headerList)
  }
  if(tocOld!==headerList){
    jsInvoke(headerList)
  }

}

//获取文本目录结构
const getToc = (cherry)=>{
  let html  = cherry.getHtml()
  const headerList = [];
  const headerRegex = /<h([1-6]).*?id="([^"]+?)".*?>(.+?)<\/h[0-6]>/g;
  html.replace(headerRegex, (match, level, id, text) => {
    let s="<a href=\"#"+id+"\">"+decodeURI(id)+"</a>"
    let id2 = decodeURI(id)
    headerList.push({ level: +level, id:id2, text:s });
    return match;
  });
  return headerList;
}

onMounted(()=>{
  init()
})
</script>

<style scoped>
:deep(#markdown-container) {
  height: v-bind(height) px;
}
</style>