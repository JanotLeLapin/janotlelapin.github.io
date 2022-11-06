<script lang="ts">
  import type { Repo } from "../lib/types";
  import Card from "./Card.svelte";

  export let repos: Repo[];
</script>

<ul class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 m-8">
  {#each repos as repo}
    <li class="transition-all"><Card {repo} /></li>
  {/each}
</ul>

<style>
  ul:has(li:hover) > li:not(:hover) {
    opacity: .7;
  }
</style>
