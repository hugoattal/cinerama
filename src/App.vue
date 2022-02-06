<template>
    <div class="app" @click="toggleFullscreen">
        <div class="cinerama">
            <div class="name">
                Cinerama
            </div>
            <div class="description">
                Click to toggle fullscreen
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

function toggleFullscreen() {
    invoke("toggle_fullscreen");
    document.body.requestFullscreen();
}
</script>

<style lang="scss">
html, body {
    margin: 0;
    padding: 0;
    background: #000000;
    font-family: Avenir, Helvetica, Arial, sans-serif;
}

.app {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    cursor: pointer;

    .cinerama {
        color: #000000;
        font-weight: 600;
        text-align: center;
        transition: color .2s ease-in-out;

        .name {
            text-transform: uppercase;
        }

        .description {
            font-size: 0.8rem;
        }
    }

    &:hover .cinerama {
        color: #404631;
    }
}
</style>
