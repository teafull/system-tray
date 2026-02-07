<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();
const greetMsg = ref("");
const name = ref("");

// 最小化窗口
async function minimizeWindow() {
  await appWindow.minimize();
}

// 关闭窗口
async function closeWindow() {
  await appWindow.close();
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="app-container">
    <!-- 自定义标题栏 -->
    <div class="titlebar" data-tauri-drag-region>
      <div class="titlebar-left">
        <span class="app-title">system-tray</span>
      </div>
      <div class="titlebar-right">
        <button
          class="titlebar-button minimize"
          @click="minimizeWindow"
          title="最小化"
          data-tauri-drag-region="false"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect x="2" y="5" width="8" height="1" fill="currentColor"/>
          </svg>
        </button>
        <button
          class="titlebar-button close"
          @click="closeWindow"
          title="关闭"
          data-tauri-drag-region="false"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M1 1L11 11M11 1L1 11" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 主内容区域 -->
    <main class="container">
      <h1>Welcome to Tauri + Vue</h1>

      <div class="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

      <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p>{{ greetMsg }}</p>
    </main>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

/* 自定义标题栏样式 */
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background: #fff;
}

.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 40px;
  background-color: #2c3e50;
  color: #ecf0f1;
  user-select: none;
  -webkit-user-select: none;
  -webkit-app-region: drag;
}

.titlebar-left {
  padding-left: 12px;
  font-size: 13px;
  font-weight: 500;
}

.app-title {
  display: flex;
  align-items: center;
  height: 100%;
}

.titlebar-right {
  display: flex;
  height: 100%;
}

.titlebar-button {
  width: 46px;
  height: 100%;
  background: transparent;
  border: none;
  color: #ecf0f1;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
  -webkit-app-region: no-drag;
}

.titlebar-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.titlebar-button.close:hover {
  background-color: #e74c3c;
}

.titlebar-button svg {
  width: 10px;
  height: 10px;
}

.container {
  flex: 1;
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  background-color: #f6f6f6;
}
</style>
<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  overflow: hidden;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
