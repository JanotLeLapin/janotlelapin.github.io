<script lang="ts">
  import type { Repo } from "../../lib/types";

  export let repo: Repo;
</script>

<a href={`https://github.com/${repo.owner}/${repo.repo}`} target="_blank" rel="noreferrer">
  <div class="relative cursor-pointer text-left text-gray-700 border border-gray-200 p-4 h-full shadow rounded hover:-translate-y-1 hover:scale-[101%] hover:shadow-lg transition-all">
    <div class="mb-6">
      <p class="text-xl">{repo.owner}/<span class="font-bold text-gray-900">{repo.repo}</span></p>
      {#if repo.description}<p>{repo.description}</p>{/if}
    </div>
    <div class="flex items-center absolute bottom-4 right-4 space-x-1">
      <p class="lang font-semibold">{repo.language}</p>
      <div class="w-3 h-3 rounded-sm" style={`background-color: ${repo.languageColor}`} />
    </div>
  </div>
</a>
