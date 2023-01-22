import {appWindow} from "@tauri-apps/api/window";

interface StorageInterface {
    //设置localStorage
    set: (key: string, value: any) => void;
    //获取localStorage,默认会自动转json
    get: (key: string, isJson?: boolean) => string | Record<any, any>;
    //是否含有key
    has: (key: string, isJson?: boolean) => boolean;
    //移除
    remove: (key: string) => void;
    //移除json对象里面的值
    removeJsonKey: (key: string,JsonKey: string) => void;
}

interface SessionStorageInterface extends StorageInterface {
    //提供session操作接口
    session: StorageInterface;
}

const storageUtil: SessionStorageInterface = {} as SessionStorageInterface;

//加方法
const extend = (s: Storage): StorageInterface => {
    return {
        set(key, value) {
            if (typeof value == "object") {
                s.setItem(key, JSON.stringify(value));
            } else {
                s.setItem(key, value);
            }
            if (value == undefined || value == null) {
                s.removeItem(key);
            }
        },
        get(key, isJson = true) {
            const item = s.getItem(key) as string;
            try {
                if (isJson) {
                    let ret =  JSON.parse(item);
                    //JSON.parse 可以转基本类型，不报错。。。，所以这里判断一下
                    if (typeof ret !== 'object') {
                        return undefined;
                    }
                    return ret;
                }
            } catch (e) {
                //解析出错,则证明不是json字符串，返回undefined
                return undefined;
            }
            return item;
        },
        has(key,isJson=false) {
            return !!(this.get(key,isJson) as string | Record<any, any>);
        },
        remove: (key) => {
            s.removeItem(key);
        },
        removeJsonKey(key,JsonKey){
            let map:any = this.get(key,true)
            delete map[JsonKey];
            this.set(key,map)
        }
    };
};

Object.assign(storageUtil, extend(window.localStorage));
Object.assign(storageUtil, {
    session: extend(window.sessionStorage),
});

export default storageUtil;
