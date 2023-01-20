// 定义
import { defineStore } from 'pinia'
import Cherry from "cherry-markdown";

export const useCherryStore = defineStore('cherryObj', {
    state: () => {
        return {
            cherry:null,
        }
    },
    // could also be defined as
    // state: () => ({ count: 0 })
    getters: {

    },
    actions: {
        setCherry(value:any){
            this.cherry = value
        },
        getCherry(){
            return this.cherry
        }
    },
})