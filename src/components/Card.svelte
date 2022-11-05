<script lang="ts">
  import type { Repo } from "../lib/types";

  export let repo: Repo;
</script>

<div class="relative cursor-pointer border border-gray-200 p-4 h-full shadow rounded hover:-translate-y-1 hover:scale-[101%] hover:shadow-lg transition-all">
  <div class="mb-6">
    <p class="text-xl">{repo.owner}/<span class="font-bold text-gray-900">{repo.repo}</span></p>
    {#if repo.description}
      <p>{repo.description}</p>
    {/if}
  </div>
  <p class="absolute bottom-4 right-4 lang font-semibold" style={`--lang-color: ${repo.languageColor}`}>{repo.language}</p>
</div>

<style>
  .lang {
    text-decoration: underline;
    text-decoration-color: var(--lang-color);
    text-decoration-thickness: 3px;
  }
</style>
