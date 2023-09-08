<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from '@tauri-apps/api/window'
const cc = async() => {
  invoke("open_docs")
}



const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name...................." />
    <button type="button" @click="greet()">Greet55555555555</button>
    <button type="button" @click="cc">Greet333</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
<style>
  .card{
    height: 100vh;
  }
</style>