<script setup>
import { ref } from 'vue';
import { pomodoro_project_backend } from 'declarations/pomodoro_project_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await pomodoro_project_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>

<template>
  <div id="app">
    <Timer />
  </div>
</template>

<script>
import Timer from './components/Timer.vue';

export default {
  name: 'App',
  components: {
    Timer
  }
}
</script>
