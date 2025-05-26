<script lang="ts">
  import { onMount } from 'svelte';
</script>

<div class="heading">
  <div>
    <h1>Joseph Daly</h1>
    <h3>Etudiant en informatique à l'UVSQ</h3>
  </div>
</div>

<style>
  .heading {
    display: grid;
    color: oklch(98.5% 0.002 247.839);
    place-items: center;
    height: 100vh;
    width: 100vw;
    text-align: center;
  }

  .heading h1 {
    font-size: 12vw;
    margin: 0;
  }

  .heading h3 {
    font-size: 2em;
    font-weight: 400;
  }
</style>
