<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import getNote from "./components/getNote.vue";
</script>

<template>
  <div class="container">
    <h1>Misskey</h1>

    <div class="row">
      <img src="https://raw.githubusercontent.com/yu256/akatsukey/master/packages/backend/assets/favicon.png">
    </div>

    <getNote />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
