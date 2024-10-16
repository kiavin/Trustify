<script setup>
import { ref } from 'vue';
import { escrowblock_backend } from 'declarations/escrowblock_backend/index';
import Hero from "../components/Hero.vue"
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await escrowblock_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}


</script>

<template>
   <Hero />
</template>

<style>
</style>