<script setup lang="ts">
import getNote from "./components/getNote.vue";
</script>

<template>
  <div class="container">
    <h1>Misskey Client</h1>

    <div class="row">
      <img src="https://raw.githubusercontent.com/yu256/akatsukey/master/packages/backend/assets/favicon.png">
    </div>

    <getNote />
  </div>
</template>