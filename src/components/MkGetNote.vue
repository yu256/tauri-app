<template>
  <div>
    <input v-model="url" placeholder="Enter an Instance URL..." />
    <button type="button" @click="setURL()">Set</button>
    <input v-model="token" placeholder="Enter a Token..." />
    <button type="button" @click="setToken()">Set</button>
    <input v-model="name" placeholder="Enter a noteId..." />
    <button type="button" @click="getNote()">Get</button>
  </div>

  <div class="note" v-if="note">
    <div class="text">
      <div class="icon">
        <img :src="user.avatarUrl">
      </div>
      {{ user.name }} {{ user.username }}@{{ user.host }}<br>{{ note }}<br>{{ createdAt }}
    </div>
    <div class="emojis" v-for="(count, reaction) in reactions" :key="reaction">
      <img v-if="isCustomEmoji(reaction)" :src="getCustomEmojiURL(reaction).url"
        :alt="getCustomEmojiURL(reaction).alt">
      <span v-else>{{ reaction }}</span>
      {{ count }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { shallowRef } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const note = shallowRef("");
const name = shallowRef("");
const url = shallowRef("");
const token = shallowRef("");
const emojis = shallowRef<Emoji[]>([]);
let reactions = new Map<string, number>()
let createdAt: string;
let user: User;

function setURL() {
  invoke("set_url", { instanceurl: url.value });
}

function setToken() {
  invoke("set_token", { token: token.value });
}

async function getNote() {
  [createdAt, user, note.value, reactions, emojis.value] = await invoke<[string, User, string, Map<string, number>, Emoji[]]>('get_note', { noteId: name.value });
}

function isCustomEmoji(reaction: string | number): boolean {
  return typeof reaction === 'string' && reaction.startsWith(':');
}

function getCustomEmojiURL(reaction: string | number): { url: string; alt: string } {
  if (typeof reaction === 'number') {
    return { url: '', alt: '' };
  }
  const emoji = emojis.value.find((emoji) => emoji.name === reaction.slice(1, -1));
  return { url: emoji!.url, alt: reaction };
}

interface Emoji {
  name: string,
  url: string,
}

interface User {
  username: string,
  host: string | null,
  name: string,
  avatarUrl: string,
  instance: Instance,
  onlineStatus: string,
}

interface Instance {
  name: string,
  softwareName: string,
  softwareVersion: string,
  iconUrl: string,
  faviconUrl: string,
  themeColor: string,
}

</script>

<style scoped>
img {
  width: 3em;
  height: 3em;
  float: left;
}

.note {
  padding: 1em;
  border: solid;
  border-radius: 2em;
  display: flex;
  overflow: clip;
}

.emojis {
  width: 3em;
  height: 3em;
  font-size: 3em;
}
</style>