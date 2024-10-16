<script setup>
import { RouterView } from 'vue-router'
import Nav from "./components/Nav.vue"
import Header from "./components/Header.vue"


</script>

<template>
  <div>
    <Header />
    <RouterView />
  </div>
</template>
