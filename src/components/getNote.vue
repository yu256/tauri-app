<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const note = ref("");
const name = ref("");
const url = ref("");

async function setURL() {
  url.value = await invoke("set_url", { instanceurl: url.value });
}

async function getNote() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  note.value = await invoke("get_note", { noteId: name.value });
}
</script>

<template>
  <div>
    <input id="greet-input" v-model="url" placeholder="Enter an Instance URL..." />
    <button type="button" @click="setURL()">Set</button>
    <br>
    <input id="greet-input" v-model="name" placeholder="Enter a noteId..." />
    <button type="button" @click="getNote()">Get</button>
  </div>

  <p>{{ note }}</p>
</template>

<style scoped>
button {
  margin-right: .3em;
}

input {
  margin: 1em;
}
</style>