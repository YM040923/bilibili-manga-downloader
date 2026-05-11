<script setup lang="ts">
import {Config} from "../bindings.ts";

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

</script>

<template>
  <n-dialog :showIcon="false"
            title="Cookie登录"
            @close="showing=false">
    <div class="flex flex-col gap-row-2" style="width: 500px">
      <n-alert type="info" :show-icon="false">
        请从浏览器DevTools(F12)中复制完整的Cookie粘贴到下方：<br/>
        (需要包含 SESSDATA, bili_jct, buvid3, dedeuserid 等关键字段)
      </n-alert>
      <n-input v-model:value="config.cookie"
               type="textarea"
               placeholder="请将浏览器中的Cookie完整的粘贴至此。&#10;例如: SESSDATA=xxx; bili_jct=xxx; buvid3=xxx; ..."
               :autosize="{ minRows: 3, maxRows: 8 }"
      />
      <n-button type="primary" block @click="showing=false">确定</n-button>
      <n-text depth="3" style="font-size: 12px">
        提示：在浏览器中打开 manga.bilibili.com，按F12打开开发者工具，在Application(应用程序) → Cookies 中找到完整Cookie，或从Network(网络)标签中复制任意请求的Cookie请求头。
      </n-text>
    </div>
  </n-dialog>
</template>