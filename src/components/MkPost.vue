<template>
    <div>
        <input v-model="note" placeholder="投稿内容">
        <button type="button" @click="post()">投稿</button>
    </div>
</template>

<script lang="ts" setup>
import { shallowRef } from "vue";
import { invoke } from "@tauri-apps/api";

const note = shallowRef("");

async function post() {
    await invoke("post", { text: note.value }) ? note.value = "" : console.log("投稿失敗");
}
</script>

<style scoped>
input {
    width: 10em;
    height: 5em;
}
</style>