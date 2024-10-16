<script setup>
   import { RouterLink } from "vue-router"
</script>


<template>
  <div class="nav nav-bar">
    <RouterLink active-class="nav-item" to="/">Home</RouterLink>
    <RouterLink active-class="nav-item" to="/about">About</RouterLink>
  </div>
</template>

<style scoped>
.active {
  font-weight: bold;
  color: lime;
}
</style>