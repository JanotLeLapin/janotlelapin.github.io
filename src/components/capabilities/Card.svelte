<script lang="ts">
  import type { Capability } from "../../lib/types";

  export let capability: Capability;
</script>

<div class="relative cursor-pointer text-left text-neutral-200 bg-neutral-800 p-4 h-full shadow-md rounded hover:-translate-y-1 hover:scale-[101%] hover:shadow-xl transition-all">
  <div class="mb-6">
    <p class="text-xl font-semibold">{capability.name}</p>
    {#if capability.description}<p>{capability.description}</p>{/if}
  </div>
</div>

