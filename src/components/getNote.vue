<script setup lang="ts">
import { shallowRef, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const note = shallowRef("");
const name = shallowRef("");
const url = shallowRef("");
let reactions = reactive(new Map<string, number>())

function setURL() {
  invoke("set_url", { instanceurl: url.value });
}

async function getNote() {
  [note.value, reactions] = await invoke<[string, Map<string, number>]>('get_note', { noteId: name.value });
}
</script>

<template>
  <div>
    <input id="input" v-model="url" placeholder="Enter an Instance URL..." />
    <button type="button" @click="setURL()">Set</button>
    <br>
    <input id="input" v-model="name" placeholder="Enter a noteId..." />
    <button type="button" @click="getNote()">Get</button>
  </div>

  <p v-if="note">
    {{ note }}
    {{ reactions }}
  </p>
</template>

<style scoped>
p {
  padding: 1em;
  border: solid;
  border-radius: 2em;
}
</style>