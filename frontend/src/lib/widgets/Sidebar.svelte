<script lang="ts">
  let { activeRoute }: {activeRoute: string} = $props()

  let routes = [
    {href: "/files", name: "Files", isActive: ()=>activeRoute.includes("/files")},
    {href: "/photos", name: "Photos", isActive: ()=>activeRoute.includes("/photos")},
    {href: "/settings", name: "Settings", isActive: ()=>activeRoute.includes("/settings")}
  ]

</script>


<div class="flex flex-col p-4 bg-gray-100 border-r-2 border-black w-full">
  <h1 class="text-2xl font-bold w-full mb-10 text-center">Cumulus</h1>
  <div class="flex flex-col gap-6 w-full">
    {#each routes as route}
      <a href={route.href} class=" underline-offset-4 pb-0] {route.isActive() ? 'w-full border-b font-bold' : 'hover:underline w-min'}">{route.name}</a>
    {/each}

  </div>
</div>