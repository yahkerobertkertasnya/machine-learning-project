<script lang="ts">
  import { fly } from "svelte/transition";
  export let imageUrl: string;
  export let title: string;
  export let text: string;
</script>

<div
  class="card w-80 min-h-[32rem] bg-white hover:bg-gray-50 shadow-xl cursor-pointer duration-300 transition-all hover:-translate-y-2 border-primary border-t-2"
  in:fly|global={{ duration: 200, y: 50 }}>
  <figure class="p-8">
    <img
      alt={title}
      src={imageUrl} />
  </figure>
  <div class="card-body items-center">
    <h2 class="card-title text-center">{title}</h2>
    <p class="text-center">{text}</p>
  </div>
</div>
