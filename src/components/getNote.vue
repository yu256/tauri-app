<script setup lang="ts">
import { shallowRef } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const note = shallowRef("");
const name = shallowRef("");
const url = shallowRef("");

function setURL() {
  invoke("set_url", { instanceurl: url.value });
}

async function getNote() {
  note.value = await invoke("get_note", { noteId: name.value });
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
  </p>
</template>

<style scoped>
p {
  padding: 1em;
  border: solid;
  border-radius: 2em;
}
</style>