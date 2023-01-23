import CherryObj from "cherry-markdown";
import {CreateTocUtil} from "./CreateTocUtil";

export default class CherryObjUtil{
    private static _cherry:CherryObj
    static interface():CherryObj{
        if(this._cherry===undefined){
            this._cherry = new CherryObj({
                id: 'markdown-container',
                value: "",
                previewer: {
                    enablePreviewerBubble: true,
                },
                callback: {
                    afterChange: this.afterChange,
                }
            })
        }
        return this._cherry
    }

    private static afterChange(text:string,html:string) {
        CreateTocUtil()
    }


}
