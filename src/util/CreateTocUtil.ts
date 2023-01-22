import {invoke} from "@tauri-apps/api";
import CherryObjUtil from "./CherryObjUtil";
import {appWindow} from "@tauri-apps/api/window";
import {ElMessage} from "element-plus";

export function CreateTocUtil():any{

    let headerList = JSON.stringify(getToc())
    jsInvoke(headerList)
}
function jsInvoke(headerList:any){

    invoke('create_toc',{json:headerList,label:appWindow.label})
        .catch(err=>{
            ElMessage.error(err)
        })
}

function getToc():any []{
    let html  = CherryObjUtil.interface().getHtml()
    let headerList:any []= [];
    const headerRegex = /<h([1-6]).*?id="([^"]+?)".*?>(.+?)<\/h[0-6]>/g;
    html.replace(headerRegex, (match, level, id, text) => {
        let s="<a href=\"#"+id+"\">"+decodeURI(id)+"</a>"
        let id2 = decodeURI(id)
        headerList.push({ level: +level, id:id2, text:s });
        return match;
    });
    return headerList;
}