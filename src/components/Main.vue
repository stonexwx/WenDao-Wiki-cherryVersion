<!--主页面-->
<template>
  <div class="common-layout">
    <el-container>
      <el-header><Menu/></el-header>
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
            <div id="markdown-container" :style="{height : height+'px'}"></div>
          </el-card>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import Cherry from "cherry-markdown";
import {onMounted, ref} from "vue";
import {appWindow} from '@tauri-apps/api/window';
import {invoke} from "@tauri-apps/api";
import {listen} from '@tauri-apps/api/event'
import 'cherry-markdown/dist/cherry-markdown.min.css'
//窗口自适应
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
      // createToc(cherry)
    },
  }
  //rust事件监听
  let cherry = new Cherry({
    id: 'markdown-container',
    value: 'valueInFo',
    previewer: {
      enablePreviewerBubble: true,
    },
    callback: {
      afterChange: callbacks.afterChange,
    },
  })
  cre.value = cherry
  createToc(cherry)
}

//侧边目录生成
const toc = ref<string>() //侧边目录
let tocOld:String

//通过rust生成目录结构
const jsInvoke = (headerList)=>{
  invoke('create_toc',{json:headerList})
      .then(res =>{
        typeof res === "string" ? toc.value = res : ""
      })
}

//创建目录
const createToc = (cherry:Cherry)=> {
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
const getToc = (cherry:Cherry)=>{
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

onMounted(() => {
  init()
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