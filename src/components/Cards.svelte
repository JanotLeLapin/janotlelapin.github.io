<script lang="ts">
  import type { Repo } from "../lib/types";
  import Card from "./Card.svelte";

  export let repos: Repo[];

  export let hovered: number = -1;
</script>

<ul class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 m-8">
  {#each repos as repo, i}
    <li style={`opacity: ${[-1, i].includes(hovered) ? 1 : .70};`} class="transition-all" on:mouseleave={_ => hovered = -1} on:mouseenter={_ => hovered = i}><Card {repo} /></li>
  {/each}
</ul>
